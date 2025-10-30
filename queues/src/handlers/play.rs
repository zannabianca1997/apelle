use std::sync::Arc;

use apelle_common::db::{SqlError, SqlTx};
use apelle_configs_dtos::{QueueUserAction, QueueUserActionSong};
use apelle_queues_dtos::events::{BuildPatchEvent as _, Collector, PatchEventBuilder};
use axum::{
    Extension, debug_handler,
    extract::Path,
    response::{IntoResponse, NoContent},
};
use reqwest::StatusCode;
use snafu::Snafu;
use tracing::instrument;
use utoipa::{IntoResponses, openapi};

use crate::{
    QueuePathParams,
    middleware::{etag::Changed, user::QueueUser},
};

#[derive(Debug, Snafu)]
pub enum PlayError {
    #[snafu(transparent)]
    SqlError {
        source: SqlError,
    },
    Forbidden,
    BadRequest,
}

impl From<sqlx::Error> for PlayError {
    fn from(value: sqlx::Error) -> Self {
        PlayError::SqlError {
            source: value.into(),
        }
    }
}

impl IntoResponse for PlayError {
    fn into_response(self) -> axum::response::Response {
        match self {
            PlayError::SqlError { source } => source.into_response(),
            PlayError::Forbidden => StatusCode::FORBIDDEN.into_response(),
            PlayError::BadRequest => StatusCode::BAD_REQUEST.into_response(),
        }
    }
}

impl IntoResponses for PlayError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [
            (
                StatusCode::FORBIDDEN.as_str().to_string(),
                openapi::Response::new("User is not allowed to start playing songs").into(),
            ),
            (
                StatusCode::BAD_REQUEST.as_str().to_string(),
                openapi::Response::new("There is no current song to start").into(),
            ),
        ]
        .into_iter()
        .chain(SqlError::responses())
        .collect()
    }
}

/// Start playing the current song
///
/// This will start playing the current song. If the song is already playing,
/// this is a no-op.
#[debug_handler(state = crate::App)]
#[utoipa::path(post, path = "/play",
responses(
    (status = StatusCode::NO_CONTENT, description = "Song started playing"),
    PlayError
),
params(QueuePathParams)
)]
#[instrument(name = "play", skip_all, fields(id = %id, user.id = %user.id()))]
pub async fn play(
    mut tx: SqlTx,
    collector: Collector<5>,
    Extension(user): Extension<Arc<QueueUser>>,
    Path(QueuePathParams { id }): Path<QueuePathParams>,
) -> Result<(Option<Changed>, NoContent), PlayError> {
    if !user.can(QueueUserAction::Song(QueueUserActionSong::Play)) {
        return Err(PlayError::Forbidden);
    }

    // Add the current song to the end of the queue
    let new_song_start_at = sqlx::query_scalar!(
        r#"
        UPDATE queue
        SET
            current_song_start_at = NOW() - (current_song_position * INTERVAL '1 second'),
            current_song_position = NULL
        WHERE
            id = $1
            AND current_song_position IS NOT NULL
        RETURNING
            current_song_start_at as "new_song_start_at!"
        "#,
        id
    )
    .fetch_optional(&mut tx)
    .await
    .map_err(SqlError::from)?;

    let Some(new_song_start_at) = new_song_start_at else {
        let current_song = sqlx::query_scalar!("SELECT current_song FROM queue WHERE id = $1", id)
            .fetch_one(&mut tx)
            .await
            .map_err(SqlError::from)?;

        if current_song.is_some() {
            // The song was already running
            return Ok((None, NoContent));
        } else {
            // There is no current song
            return Err(PlayError::BadRequest);
        }
    };

    PatchEventBuilder::queue(id)
        .add("/current/starts_at", new_song_start_at)
        .remove("/current/position")
        .build()
        .collect(&collector)
        .await;

    Ok((
        Some(Changed::new(&mut tx, &collector, id).await?),
        NoContent,
    ))
}
