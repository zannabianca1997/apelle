use std::{convert::identity, mem};

use apelle_common::{
    AuthHeaders, Reporter, TracingClient,
    common_errors::{CacheError, SQLError, SQLSnafu},
};
use apelle_songs_dtos::{provider::ResolveResponse, public::ResolveSongRequest};
use axum::{
    Json,
    body::Body,
    debug_handler,
    extract::State,
    response::{IntoResponse, Redirect},
};
use futures::TryFutureExt as _;
use redis::aio::ConnectionManager;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
use sqlx::PgPool;
use textwrap_macros::unfill;
use url::Url;
use uuid::Uuid;

use crate::providers::{provider_for_urn, resolve_endpoint, solved_endpoint};

#[derive(Debug, Snafu)]
pub enum ResolveSongError {
    #[snafu(transparent)]
    SQLError {
        source: SQLError,
    },
    #[snafu(transparent)]
    CacheError {
        source: CacheError,
    },

    ClientError {
        response: reqwest::Response,
    },

    BadGatewayError {
        provider: Url,
        source: reqwest::Error,
    },
}
impl IntoResponse for ResolveSongError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ResolveSongError::SQLError { source } => source.into_response(),
            ResolveSongError::CacheError { source } => source.into_response(),
            ResolveSongError::ClientError {
                response: mut reqwest_response,
            } => {
                let mut response_builder =
                    axum::http::Response::builder().status(reqwest_response.status());
                *response_builder.headers_mut().unwrap() =
                    mem::take(reqwest_response.headers_mut());

                // Directly stream the response
                response_builder
                    .body(Body::from_stream(reqwest_response.bytes_stream()))
                    // This unwrap is fine because the body is empty here
                    .unwrap()
            }
            ResolveSongError::BadGatewayError { provider, source } => {
                tracing::error!(%provider,"Bad gateway: {}", Reporter(source));
                StatusCode::BAD_GATEWAY.into_response()
            }
        }
    }
}

#[debug_handler(state=crate::App)]
pub async fn resolve(
    State(db): State<PgPool>,
    State(mut cache): State<ConnectionManager>,
    client: TracingClient,
    user: AuthHeaders,
    Json(ResolveSongRequest { source_urn, data }): Json<ResolveSongRequest>,
) -> Result<Redirect, ResolveSongError> {
    let provider = provider_for_urn(&mut cache, &source_urn).await?;

    let resp = client
        .post(resolve_endpoint(&provider))
        .query(&[("public", "false")])
        .json(&data)
        .send()
        .and_then(async |response| {
            // TODO: If the response indicate that the provider is down, remove it from the cache

            // Propagate the client errors
            if response.status().is_client_error() {
                return Ok(Err(ResolveSongError::ClientError { response }));
            }

            // Raise for other errors and parse the body
            response
                .error_for_status()?
                .json::<ResolveResponse>()
                .await
                .map(Ok)
        })
        .await
        .with_context(|_| BadGatewaySnafu {
            provider: provider.clone(),
        })
        .and_then(identity)?;

    let (title, duration, callback) = match resp {
        ResolveResponse::New {
            title,
            duration,
            public: _,
            callback,
        } => (title, duration, callback),
        ResolveResponse::Existing { id, public: _ } => {
            return Ok(redirect(id));
        }
    };

    // New song, creating the main entity
    let id: Uuid = sqlx::query_scalar(unfill!(
        "
        WITH used_source AS (
            UPDATE source
            SET last_heard = NOW()
            WHERE urn = $1
            RETURNING id
        )
        INSERT INTO song (duration, title, added_by, source_id)
        SELECT $2, $3, $4, used_source.id
        FROM used_source
        RETURNING id
        "
    ))
    .bind(&source_urn)
    .bind(duration.num_seconds() as i32)
    .bind(title)
    .bind(user.id())
    .fetch_one(&db)
    .await
    .context(SQLSnafu)?;

    // Telling the provider that we added the song, enabling it to save the data

    let mut cb = client.put(solved_endpoint(&provider, id));
    // add the callback if provided
    if let Some(callback) = callback {
        cb = cb.json(&callback);
    }
    let status = cb
        .send()
        .await
        .and_then(|r| r.error_for_status().map(|r| r.status()));

    match status {
        Ok(StatusCode::CREATED) => {}
        Ok(returned_status) => {
            tracing::warn!(
                %returned_status,
                %provider,
                "Provider told us to create the song, but then said the song already existed"
            );
        }
        Err(err) => {
            // Failed put, reverting the creation of the main entity
            tracing::error!(%provider, "Error from provider, reverting creation of song");
            let deletion_result = sqlx::query("DELETE FROM song WHERE id = $1")
                .bind(id)
                .execute(&db)
                .await;

            // If the deletion failed, log it
            // This is critical, but we want to report the original error
            match deletion_result {
                Err(err) => {
                    tracing::error!(song_id = %id,"Failed to delete the song: {}", Reporter(err))
                }
                Ok(r) if r.rows_affected() == 0 => {
                    tracing::error!(song_id = %id,"Failed to delete the song, no rows affected");
                }
                Ok(_) => {}
            }

            // Propagate the original error
            return Err(ResolveSongError::BadGatewayError {
                provider,
                source: err,
            });
        }
    }

    Ok(redirect(id))
}

fn redirect(id: Uuid) -> Redirect {
    Redirect::to(&format!("solved/{id}"))
}
