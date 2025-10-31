use std::sync::Arc;

use apelle_common::db::{SqlError, SqlTx};
use apelle_configs_dtos::{QueueUserAction, QueueUserActionSong};
use apelle_queues_dtos::events::{BuildPatchEvent as _, Collector, QueueEventBuilder};
use axum::{
    Extension, debug_handler,
    extract::Path,
    response::{IntoResponse, NoContent},
};
use reqwest::StatusCode;
use snafu::Snafu;
use utoipa::{IntoResponses, openapi};

use crate::{
    QueuedSongPathParams,
    middleware::{etag::Changed, user::QueueUser},
};

#[derive(Debug, Snafu)]
pub enum RemoveSongError {
    #[snafu(transparent)]
    SqlError {
        source: SqlError,
    },
    Forbidden,
    NotFound,
}

impl From<sqlx::Error> for RemoveSongError {
    fn from(value: sqlx::Error) -> Self {
        RemoveSongError::SqlError {
            source: value.into(),
        }
    }
}

impl IntoResponse for RemoveSongError {
    fn into_response(self) -> axum::response::Response {
        match self {
            RemoveSongError::SqlError { source } => source.into_response(),
            RemoveSongError::Forbidden => StatusCode::FORBIDDEN.into_response(),
            RemoveSongError::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

impl IntoResponses for RemoveSongError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [
            (
                StatusCode::FORBIDDEN.as_str().to_string(),
                openapi::Response::new("User is not allowed to remove songs").into(),
            ),
            (
                StatusCode::NOT_FOUND.as_str().to_string(),
                openapi::Response::new("The song is not in the queue").into(),
            ),
        ]
        .into_iter()
        .chain(SqlError::responses())
        .collect()
    }
}

/// Remove a song from the queue
#[debug_handler(state = crate::App)]
#[utoipa::path(post, path = "/",
responses(
    (status = StatusCode::NO_CONTENT, description = "Song removed"),
    RemoveSongError
),
params(QueuedSongPathParams)
)]
pub async fn remove_song(
    mut tx: SqlTx,
    collector: Collector,
    Extension(user): Extension<Arc<QueueUser>>,
    Path(QueuedSongPathParams { queue, song }): Path<QueuedSongPathParams>,
) -> Result<(Changed, NoContent), RemoveSongError> {
    if !user.can(QueueUserAction::Song(QueueUserActionSong::Remove)) {
        return Err(RemoveSongError::Forbidden);
    }

    let deleted = sqlx::query!(
        r"
        DELETE FROM queued_song
        WHERE queue_id = $1 AND song_id = $2
        ",
        queue,
        song
    )
    .execute(&mut tx)
    .await
    .map_err(SqlError::from)?
    .rows_affected();

    if deleted == 0 {
        return Err(RemoveSongError::NotFound);
    }

    QueueEventBuilder::new(queue)
        .remove(format!("/queue/{song}"))
        .build()
        .collect(&collector)
        .await;

    Ok((
        Changed::change(&mut tx, &collector, queue).await?,
        NoContent,
    ))
}
