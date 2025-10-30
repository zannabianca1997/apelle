use std::sync::Arc;

use apelle_common::db::{SqlError, SqlTx};
use apelle_configs_dtos::{QueueUserAction, QueueUserActionSong};
use apelle_queues_dtos::events::{BuildPatchEvent as _, Collector, PatchEventBuilder};
use axum::{
    Extension, debug_handler,
    extract::Path,
    response::{IntoResponse, NoContent},
};
use chrono::Duration;
use reqwest::StatusCode;
use snafu::Snafu;
use tracing::instrument;
use utoipa::{IntoResponses, openapi};

use crate::{
    QueuePathParams,
    middleware::{etag::Changed, user::QueueUser},
};

#[derive(Debug, Snafu)]
pub enum PauseError {
    #[snafu(transparent)]
    SqlError {
        source: SqlError,
    },
    Forbidden,
    BadRequest,
}

impl From<sqlx::Error> for PauseError {
    fn from(value: sqlx::Error) -> Self {
        PauseError::SqlError {
            source: value.into(),
        }
    }
}

impl IntoResponse for PauseError {
    fn into_response(self) -> axum::response::Response {
        match self {
            PauseError::SqlError { source } => source.into_response(),
            PauseError::Forbidden => StatusCode::FORBIDDEN.into_response(),
            PauseError::BadRequest => StatusCode::BAD_REQUEST.into_response(),
        }
    }
}

impl IntoResponses for PauseError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [
            (
                StatusCode::FORBIDDEN.as_str().to_string(),
                openapi::Response::new("User is not allowed to pause songs").into(),
            ),
            (
                StatusCode::BAD_REQUEST.as_str().to_string(),
                openapi::Response::new("There is no current song to pause").into(),
            ),
        ]
        .into_iter()
        .chain(SqlError::responses())
        .collect()
    }
}

/// Pause the current song
///
/// This will pause the current song. If the song is already paused,
/// this is a no-op.
#[debug_handler(state = crate::App)]
#[utoipa::path(post, path = "/pause",
responses(
    (status = StatusCode::NO_CONTENT, description = "Song paused"),
    PauseError
),
params(QueuePathParams)
)]
#[instrument(name = "pause", skip_all, fields(id = %id, user.id = %user.id()))]
pub async fn pause(
    mut tx: SqlTx,
    collector: Collector<5>,
    Extension(user): Extension<Arc<QueueUser>>,
    Path(QueuePathParams { id }): Path<QueuePathParams>,
) -> Result<(Option<Changed>, NoContent), PauseError> {
    if !user.can(QueueUserAction::Song(QueueUserActionSong::Pause)) {
        return Err(PauseError::Forbidden);
    }

    let new_song_position = sqlx::query_scalar!(
        r#"
        UPDATE queue
        SET
            current_song_position = EXTRACT(EPOCH FROM (NOW() - current_song_start_at))::INTEGER,
            current_song_start_at = NULL
        WHERE
            id = $1
            AND current_song_start_at IS NOT NULL
        RETURNING
            current_song_position as "new_song_position!"
        "#,
        id
    )
    .fetch_optional(&mut tx)
    .await
    .map_err(SqlError::from)?
    .map(|x| Duration::seconds(x as _));

    let Some(new_song_position) = new_song_position else {
        let current_song = sqlx::query_scalar!("SELECT current_song FROM queue WHERE id = $1", id)
            .fetch_one(&mut tx)
            .await
            .map_err(SqlError::from)?;

        if current_song.is_some() {
            // The song was already paused
            return Ok((None, NoContent));
        } else {
            // There is no current song
            return Err(PauseError::BadRequest);
        }
    };

    PatchEventBuilder::queue(id)
        .add("/current/position", new_song_position.to_string())
        .remove("/current/starts_at")
        .build()
        .collect(&collector)
        .await;

    Ok((
        Some(Changed::new(&mut tx, &collector, id).await?),
        NoContent,
    ))
}
