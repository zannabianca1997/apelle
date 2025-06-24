use std::sync::Arc;

use apelle_common::{
    Reporter, ServicesClient,
    common_errors::{SQLError, SQLSnafu},
    id_or_rep::IdOrRep,
};
use apelle_configs_dtos::{QueueConfig, QueueUserAction, QueueUserActionQueue};
use apelle_songs_dtos::public::{SolvedQueryParams, Song};
use axum::{
    Extension, Json, debug_handler,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use chrono::Duration;
use futures::{
    FutureExt, StreamExt, TryFutureExt, TryStreamExt as _, future::OptionFuture, stream,
};
use reqwest::StatusCode;
use snafu::{ResultExt as _, Snafu};
use sqlx::{PgPool, Row as _};
use textwrap_macros::unfill;
use utoipa::{IntoParams, IntoResponses, openapi};
use uuid::Uuid;

use crate::{
    QueuePathParams, Services,
    middleware::user::QueueUser,
    model::{Current, Queue, QueuedSong},
};

#[derive(Debug, Snafu)]
pub enum GetError {
    #[snafu(transparent)]
    SqlError {
        source: SQLError,
    },
    #[snafu(transparent)]
    BadGateway {
        source: reqwest::Error,
    },
    QueueNotFound,
    Forbidden,
}

impl IntoResponse for GetError {
    fn into_response(self) -> axum::response::Response {
        match self {
            GetError::SqlError { source } => source.into_response(),
            GetError::BadGateway { source } => {
                tracing::error!("Bad gateway: {}", Reporter(source));
                StatusCode::BAD_GATEWAY.into_response()
            }
            GetError::QueueNotFound => StatusCode::NOT_FOUND.into_response(),
            GetError::Forbidden => StatusCode::FORBIDDEN.into_response(),
        }
    }
}

impl IntoResponses for GetError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [
            (
                StatusCode::NOT_FOUND.as_str().to_string(),
                openapi::Response::new("Queue not found").into(),
            ),
            (
                StatusCode::FORBIDDEN.as_str().to_string(),
                openapi::Response::new("User is not allowed to read the queue data").into(),
            ),
            (
                StatusCode::BAD_GATEWAY.as_str().to_string(),
                openapi::Response::new("Error returned from songs service").into(),
            ),
        ]
        .into_iter()
        .chain(SQLError::responses())
        .collect()
    }
}

#[derive(serde::Deserialize, IntoParams)]
pub struct GetQueryParams {
    /// Return the full queue config instead of just the UUID
    #[serde(default)]
    pub config: bool,
    /// Return the full song data instead of just the UUID
    #[serde(default)]
    pub songs: bool,
    // For each song, return the source data in addition to the song data (like
    // thumbnails or public url)
    #[serde(default)]
    pub songs_source: bool,
}

async fn solve_song(
    client: ServicesClient,
    services: Arc<Services>,
    id: Uuid,
    source_data: bool,
) -> Result<Song, GetError> {
    Ok(client
        .get(services.songs_url.join(&format!("solved/{id}")).unwrap())
        .query(&SolvedQueryParams { source_data })
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

/// Read the queue data
#[debug_handler(state = crate::App)]
#[utoipa::path(get, path = "/",
    responses(
        (status = StatusCode::OK, description = "Queue created", content_type = "application/json", body = Queue),
        GetError
    ),
    params(GetQueryParams, QueuePathParams)
)]
pub async fn get(
    State(db): State<PgPool>,
    client: ServicesClient,
    State(services): State<Arc<Services>>,
    Extension(user): Extension<Arc<QueueUser>>,
    Extension(config): Extension<Arc<QueueConfig>>,
    Query(GetQueryParams {
        config: return_config,
        songs: return_songs,
        songs_source: return_songs_source,
    }): Query<GetQueryParams>,
    Path(QueuePathParams { id }): Path<QueuePathParams>,
) -> Result<Json<Queue>, GetError> {
    if !user.can(QueueUserAction::Queue(QueueUserActionQueue::Get)) {
        return Err(GetError::Forbidden);
    }

    let current_client = client.clone();
    let current_services = services.clone();

    let queue = sqlx::query_as(
        unfill!(
            "
            SELECT 
                code, 
                current_song, current_song_position, current_song_start_at, player_state_id, 
                created, updated 
            FROM queue WHERE id = $1
            "
        )
        .trim_ascii(),
    )
    .bind(id)
    .fetch_one(&db)
    .map(|r| r.context(SQLSnafu).map_err(GetError::from))
    .and_then(
        |(
            code,
            current_song,
            current_song_position,
            current_song_start_at,
            player_state_id,
            created,
            updated,
        )| async move {
            let current_song = OptionFuture::from(Option::map(current_song, |current: Uuid| {
                solve_song(
                    current_client,
                    current_services,
                    current,
                    return_songs_source,
                )
            }))
            .await
            .transpose()?;

            let current_song_position = Option::map(current_song_position, Duration::seconds);

            let current = match (current_song, current_song_position, current_song_start_at) {
                (Some(song), Some(position), None) => {
                    Some(Current::stopped(song, player_state_id, position))
                }
                (Some(song), None, Some(starts_at)) => {
                    Some(Current::playing(song, player_state_id, starts_at))
                }
                (None, None, None) => None,
                _ => panic!(
                    "Invalid database state: the check on the table should have avoided this state"
                ),
            };

            Ok((code, current, created, updated))
        },
    );

    let songs = sqlx::query(
        unfill!(
            "
            SELECT
                qs.song_id,
                qs.queued_at,
                COALESCE(tl.likes_count, 0) AS likes,
                COALESCE(ul.user_likes_count, 0) AS user_likes
            FROM
                queued_song qs
            LEFT JOIN LATERAL (
                SELECT
                    l.queue_id,
                    l.song_id,
                    SUM(l.count) AS likes_count
                FROM
                    likes l
                WHERE
                    l.queue_id = qs.queue_id AND l.song_id = qs.song_id
                GROUP BY
                    l.queue_id,
                    l.song_id
            ) tl ON TRUE
            LEFT JOIN LATERAL (
                SELECT
                    l.queue_id,
                    l.song_id,
                    SUM(l.count) AS user_likes_count
                FROM
                    likes l
                WHERE
                    l.queue_id = qs.queue_id
                    AND l.song_id = qs.song_id
                    AND l.user_id = $2
                GROUP BY
                    l.queue_id,
                    l.song_id
            ) ul ON TRUE
            WHERE
                qs.queue_id = $1
            "
        )
        .trim_ascii(),
    )
    .bind(id)
    .bind(user.user().id())
    .map(|row| {
        let id = row.get("song_id");
        (
            id,
            QueuedSong {
                song: IdOrRep::Id(id),
                queued_at: row.get("queued_at"),
                likes: row.get::<i16, _>("likes") as _,
                user_likes: row.get::<i16, _>("user_likes") as _,
            },
        )
    })
    .fetch(&db)
    .map(|r| r.context(SQLSnafu).map_err(GetError::from));

    let songs = if return_songs {
        songs
            .map(|r| {
                let client = client.clone();
                let services = services.clone();
                stream::once(Box::pin(async move {
                    let mut r = r?;
                    let (_, QueuedSong { song, .. }) = &mut r;
                    song.or_try_extract_inplace(|id| {
                        solve_song(client, services, id, return_songs_source)
                    })
                    .await?;
                    Ok(r)
                }))
            })
            .flatten_unordered(None)
            .try_collect()
            .left_future()
    } else {
        songs.try_collect().right_future()
    };

    let ((code, current, created, updated), queue) = tokio::try_join!(queue, songs)?;

    Ok(Json(Queue {
        id,
        code,
        current,
        config: if return_config {
            IdOrRep::Rep((*config).clone())
        } else {
            IdOrRep::Id(config.id)
        },
        queue,
        created,
        updated,
    }))
}
