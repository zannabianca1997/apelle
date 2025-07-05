use std::sync::Arc;

use apelle_common::{
    Reporter, ServicesClient,
    db::{SqlError, SqlTx},
    id_or_rep::{HasId as _, IdOrRep},
};
use apelle_configs_dtos::{QueueUserAction, QueueUserActionSong};
use apelle_queues_events::events::{BuildPatchEvent as _, Collector, Event};
use apelle_songs_dtos::public::{
    ResolveSongRequest, SearchResponseItemState, SolvedQueryParams, Song,
};
use axum::{
    Extension, Json, debug_handler,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use serde::Deserialize;
use snafu::Snafu;
use textwrap_macros::unfill;
use utoipa::{IntoParams, IntoResponses, ToSchema, openapi};

use crate::{
    QueuePathParams, Services,
    middleware::{etag::Changed, user::QueueUser},
    model::QueuedSong,
};

#[derive(Debug, Snafu)]
pub enum EnqueueError {
    #[snafu(transparent)]
    SqlError {
        source: SqlError,
    },
    #[snafu(transparent)]
    BadGateway {
        source: reqwest::Error,
    },
    Forbidden,
    Conflict,
}

impl From<sqlx::Error> for EnqueueError {
    fn from(value: sqlx::Error) -> Self {
        EnqueueError::SqlError {
            source: value.into(),
        }
    }
}

impl IntoResponse for EnqueueError {
    fn into_response(self) -> axum::response::Response {
        match self {
            EnqueueError::SqlError { source } => source.into_response(),
            EnqueueError::BadGateway { source } => {
                tracing::error!("Bad gateway: {}", Reporter(source));
                StatusCode::BAD_GATEWAY.into_response()
            }
            EnqueueError::Forbidden => StatusCode::FORBIDDEN.into_response(),
            EnqueueError::Conflict => StatusCode::CONFLICT.into_response(),
        }
    }
}

impl IntoResponses for EnqueueError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [
            (
                StatusCode::FORBIDDEN.as_str().to_string(),
                openapi::Response::new("User is not allowed to enqueue songs").into(),
            ),
            (
                StatusCode::BAD_GATEWAY.as_str().to_string(),
                openapi::Response::new("Error returned from songs service").into(),
            ),
            (
                StatusCode::CONFLICT.as_str().to_string(),
                openapi::Response::new("The song is already in the queue").into(),
            ),
        ]
        .into_iter()
        .chain(SqlError::responses())
        .collect()
    }
}

#[derive(serde::Deserialize, IntoParams)]
pub struct EnqueueQueryParams {
    /// Override the default behavior of auto-liking
    #[serde(default)]
    pub autolike: Option<bool>,
    /// Return the full song data instead of just the UUID
    #[serde(default)]
    pub song: bool,
    /// Return also the source data for the song
    #[serde(default)]
    pub song_source: bool,
}

#[derive(Debug, Deserialize, Clone, ToSchema)]
pub struct SearchResponseItem {
    /// Source that provided this search result
    pub source: String,
    /// Data to pass the service to resolve the song
    pub state: SearchResponseItemState,
}

/// Read the queue data
#[debug_handler(state = crate::App)]
#[utoipa::path(post, path = "/enqueue",
responses(
    (status = StatusCode::OK, description = "Song enqueued", content_type = "application/json", body = QueuedSong),
    EnqueueError
),
params(EnqueueQueryParams, QueuePathParams)
)]
pub async fn enqueue(
    mut tx: SqlTx,
    mut collector: Collector<5>,
    client: ServicesClient,
    State(services): State<Arc<Services>>,
    Extension(user): Extension<Arc<QueueUser>>,
    Query(EnqueueQueryParams {
        autolike,
        song: return_song,
        song_source: return_song_source,
    }): Query<EnqueueQueryParams>,
    Path(QueuePathParams { id }): Path<QueuePathParams>,
    Json(search_response): Json<SearchResponseItem>,
) -> Result<(Changed, Json<QueuedSong>), EnqueueError> {
    if !user.can(QueueUserAction::Song(QueueUserActionSong::Enqueue)) {
        return Err(EnqueueError::Forbidden);
    }

    // Resolve the song
    let song: Song = match search_response {
        SearchResponseItem {
            state: SearchResponseItemState::Known { id },
            ..
        } => client.get(services.songs_url.join(&format!("solved/{id}")).unwrap()),
        SearchResponseItem {
            source,
            state: SearchResponseItemState::New { data },
        } => client
            .post(services.songs_url.join("resolve").unwrap())
            .json(&ResolveSongRequest { source, data }),
    }
    .query(&SolvedQueryParams {
        source_data: return_song && return_song_source,
    })
    .send()
    .await?
    .error_for_status()?
    .json()
    .await?;

    // Insert the song in the database queue
    let (queued_at, user_likes) = async {
        // Add the song to the queue
        let queued_at = sqlx::query_scalar(
            unfill!(
                "
                INSERT INTO queued_song (queue_id, song_id, queued_by)
                VALUES ($1, $2, $3)
                RETURNING queued_at
                "
            )
            .trim_ascii(),
        )
        .bind(id)
        .bind(song.id)
        .bind(user.id())
        .fetch_one(&mut tx)
        .await
        .map_err(|err| {
            if err
                .as_database_error()
                .is_some_and(|err| err.constraint() == Some("pk_queued_song"))
            {
                EnqueueError::Conflict
            } else {
                err.into()
            }
        })?;

        // If auto-like is enabled and the user has likes available, give them
        // one
        let user_likes =
            if autolike.unwrap_or(user.auto_like()) && user.likes() < user.role().max_likes {
                sqlx::query("INSERT INTO likes (queue_id, song_id, user_id) VALUES ($1, $2, $3)")
                    .bind(id)
                    .bind(song.id)
                    .bind(user.id())
                    .execute(&mut tx)
                    .await?;
                1
            } else {
                0
            };

        Ok::<_, EnqueueError>((queued_at, user_likes))
    }
    .await?;

    let queued_song = QueuedSong {
        song: if return_song {
            IdOrRep::Rep(song)
        } else {
            IdOrRep::Id(song.id)
        },
        queued_at,
        likes: user_likes,
        user_likes,
    };

    Event::queue(id)
        .add(
            format!("/queue/{}", queued_song.song.id()),
            QueuedSong {
                song: IdOrRep::Id(queued_song.song.id()),
                user_likes: 0,
                ..queued_song.clone()
            },
        )
        .build()
        .collect(&collector)
        .await;

    if user_likes != 0 {
        Event::user(id, user.id())
            .replace(
                format!("/queue/{}/user_likes", queued_song.song.id()),
                user_likes,
            )
            .build()
            .collect(&mut collector)
            .await;
    }

    Ok((
        Changed::new(&mut tx, &collector, id).await?,
        Json(queued_song),
    ))
}
