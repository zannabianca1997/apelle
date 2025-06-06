use apelle_common::{
    Reporter, TracingClient,
    common_errors::{CacheError, SQLError, SQLSnafu},
};
use apelle_songs_dtos::public::Song;
use axum::{
    Json, debug_handler,
    extract::{Path, Query, State},
    response::{IntoResponse, NoContent},
};
use chrono::Duration;
use futures::TryFutureExt;
use redis::aio::ConnectionManager;
use reqwest::StatusCode;
use serde::Deserialize;
use snafu::{ResultExt as _, Snafu};
use sqlx::PgPool;
use textwrap_macros::unfill;
use uuid::Uuid;

use crate::providers::{provider_for_urn, solved_endpoint};

#[derive(Debug, Snafu)]
pub enum Error {
    NotFound,
    #[snafu(transparent)]
    SQLError {
        source: SQLError,
    },
    #[snafu(transparent)]
    CacheError {
        source: CacheError,
    },
    BadGatewayError {
        provider: String,
        source: reqwest::Error,
    },
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::NotFound => StatusCode::NOT_FOUND.into_response(),
            Error::SQLError { source } => source.into_response(),
            Error::CacheError { source } => source.into_response(),
            Error::BadGatewayError { provider, source } => {
                tracing::error!(%provider,"Bad gateway: {}", Reporter(source));
                StatusCode::BAD_GATEWAY.into_response()
            }
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct GetQueryParams {
    #[serde(default = "default_true")]
    source_data: bool,
}

#[debug_handler(state=crate::App)]
pub async fn get(
    State(db): State<PgPool>,
    State(mut cache): State<ConnectionManager>,
    client: TracingClient,
    Path(id): Path<Uuid>,
    Query(GetQueryParams { source_data }): Query<GetQueryParams>,
) -> Result<Json<Song>, Error> {
    tracing::debug!(%id, "Getting song data");

    let (title, duration, added_by, created, source_urn): (_, i32, _, _, String) =
        sqlx::query_as(unfill!(
            "
            SELECT song.title, song.duration, song.added_by, song.created, source.urn
            FROM song
            JOIN source ON song.source_id = source.id
            WHERE song.id = $1
            "
        ))
        .bind(id)
        .fetch_optional(&db)
        .await
        .context(SQLSnafu)?
        .ok_or(Error::NotFound)?;

    let provider = provider_for_urn(&mut cache, &source_urn).await?;

    let source_data = if source_data {
        Some(
            client
                .get(solved_endpoint(&provider, id))
                .send()
                .and_then(async |r| r.error_for_status()?.json().await)
                .await
                .with_context(|_| BadGatewaySnafu {
                    provider: provider.to_string(),
                })?,
        )
    } else {
        None
    };

    Ok(Json(Song {
        id,
        title,
        duration: Duration::seconds(duration as _),
        added_by,
        created,
        source_data,
    }))
}

#[debug_handler(state=crate::App)]
pub async fn delete(
    State(db): State<PgPool>,
    State(mut cache): State<ConnectionManager>,
    client: TracingClient,
    Path(id): Path<Uuid>,
) -> Result<NoContent, Error> {
    tracing::info!(%id, "Deleting song");

    let source_urn: String = sqlx::query_scalar(unfill!(
        "
        WITH deleted_song AS (
            DELETE FROM song
            WHERE id = $1
            RETURNING source_id
        )
        SELECT source.urn
        FROM source
        JOIN deleted_song ON source.id = deleted_song.source_id;
        "
    ))
    .bind(id)
    .fetch_optional(&db)
    .await
    .context(SQLSnafu)?
    .ok_or(Error::NotFound)?;

    let provider = provider_for_urn(&mut cache, &source_urn).await?;

    // Tell the provider to delete the song
    client
        .delete(solved_endpoint(&provider, id))
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .with_context(|_| BadGatewaySnafu {
            provider: provider.to_string(),
        })?;

    Ok(NoContent)
}
