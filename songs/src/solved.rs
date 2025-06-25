use apelle_common::{
    Reporter, ServicesClient,
    common_errors::CacheError,
    db::{SqlError, SqlTx},
};
use apelle_songs_dtos::public::{SolvedQueryParams, Song};
use axum::{
    Json, debug_handler,
    extract::{Path, Query, State},
    response::{IntoResponse, NoContent},
};
use axum_extra::{
    TypedHeader,
    headers::{CacheControl, Header as _},
};
use chrono::Duration;
use futures::TryFutureExt;
use redis::aio::ConnectionManager;
use reqwest::StatusCode;
use snafu::{ResultExt as _, Snafu};
use textwrap_macros::unfill;
use utoipa::{IntoResponses, openapi};
use uuid::Uuid;

use crate::{
    providers::{provider_for_urn, solved_endpoint},
    seen_sources::SeenSourcesWorker,
};

#[derive(Debug, Snafu)]
pub enum Error {
    NotFound,
    #[snafu(transparent)]
    Sql {
        source: SqlError,
    },
    #[snafu(transparent)]
    Cache {
        source: CacheError,
    },
    BadGateway {
        provider: String,
        source: reqwest::Error,
    },
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::NotFound => StatusCode::NOT_FOUND.into_response(),
            Error::Sql { source } => source.into_response(),
            Error::Cache { source } => source.into_response(),
            Error::BadGateway { provider, source } => {
                tracing::error!(%provider,"Bad gateway: {}", Reporter(source));
                StatusCode::BAD_GATEWAY.into_response()
            }
        }
    }
}

impl IntoResponses for Error {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [(
            StatusCode::BAD_GATEWAY.as_str().to_string(),
            openapi::Response::new("Error returned from provider").into(),
        )]
        .into_iter()
        .chain(SqlError::responses())
        .chain(CacheError::responses())
        .collect()
    }
}

/// Get song data
///
/// Get the data of a song that was previously resolved. If `source_data` is set
/// to `true`, the data from the song source will be included (like thumbnails
/// and url).
#[debug_handler(state=crate::App)]
#[utoipa::path(get, path = "/solved/{id}", 
    responses(
        (status = StatusCode::OK, description = "Song data", content_type = "application/json", body = Song), 
        Error
    ),
    params(SolvedQueryParams),
)]
pub async fn get(
    mut tx: SqlTx,
    State(mut cache): State<ConnectionManager>,
    State(seen_sources): State<SeenSourcesWorker>,
    client: ServicesClient,
    Path(id): Path<Uuid>,
    Query(SolvedQueryParams { source_data }): Query<SolvedQueryParams>,
) -> Result<(TypedHeader<CacheControl>, Json<Song>), Error> {
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
        .fetch_optional(&mut tx)
        .await
        .map_err(SqlError::from)?
        .ok_or(Error::NotFound)?;

    // The song data in the database won't be updated
    let mut cache_control = CacheControl::new()
        .with_immutable()
        .with_max_age(std::time::Duration::from_secs(31536000));

    let source_data = if source_data {
        let provider = provider_for_urn(&mut cache, source_urn.as_str()).await?;

        let (response_cache, response) = client
            .get(solved_endpoint(&provider, id))
            .send()
            .and_then(async |r| {
                let r = r.error_for_status()?;

                let cache_control =
                    CacheControl::decode(&mut r.headers().get_all(CacheControl::name()).iter())
                        .unwrap_or(cache_control);

                let content = r.json().await?;

                Ok((cache_control, content))
            })
            .await
            .with_context(|_| BadGatewaySnafu {
                provider: provider.to_string(),
            })?;

        // Using the cache control header of the response, as it is surely more
        // restrictive
        cache_control = response_cache;

        // Marking that we seen the source
        seen_sources.seen_urn(source_urn).await;

        Some(response)
    } else {
        None
    };

    Ok((
        TypedHeader(cache_control),
        Json(Song {
            id,
            title,
            duration: Duration::seconds(duration as _),
            added_by,
            created,
            source_data,
        }),
    ))
}

/// Delete a song
///
/// This will delete the song from the database and ask the provider to delete
/// the song.
#[debug_handler(state=crate::App)]
#[utoipa::path(delete, path = "/solved/{id}", 
    responses(
        (status = StatusCode::NO_CONTENT, description = "Song deleted"),
        Error
    )
)]
pub async fn delete(
    mut tx: SqlTx,
    State(mut cache): State<ConnectionManager>,
    State(seen_sources): State<SeenSourcesWorker>,
    client: ServicesClient,
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
    .fetch_optional(&mut tx)
    .await
    .map_err(SqlError::from)?
    .ok_or(Error::NotFound)?;

    let provider = provider_for_urn(&mut cache, source_urn.as_str()).await?;

    // Tell the provider to delete the song
    client
        .delete(solved_endpoint(&provider, id))
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .with_context(|_| BadGatewaySnafu {
            provider: provider.to_string(),
        })?;

    // Marking that we seen the source
    seen_sources.seen_urn(source_urn).await;

    Ok(NoContent)
}
