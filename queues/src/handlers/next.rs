use std::sync::Arc;

use apelle_common::{
    Reporter, ServicesClient,
    db::{SqlError, SqlTx},
    id_or_rep::IdOrRep,
};
use apelle_configs_dtos::{QueueUserAction, QueueUserActionSong};
use apelle_queues_events::events::{BuildPatchEvent as _, Collector, PatchEventBuilder};
use apelle_songs_dtos::public::{SolvedQueryParams, Song};
use axum::{
    Extension, debug_handler,
    extract::{Path, Query, State},
    response::{IntoResponse, NoContent},
};
use chrono::{DateTime, FixedOffset};
use reqwest::StatusCode;
use snafu::Snafu;
use textwrap_macros::unfill;
use tracing::instrument;
use utoipa::{IntoParams, IntoResponses, openapi};
use uuid::Uuid;

use crate::{
    QueuePathParams, Services,
    middleware::{etag::Changed, user::QueueUser},
    model::{Current, QueuedSong},
};

#[derive(Debug, Snafu)]
pub enum NextError {
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
    NotFound,
}

impl From<sqlx::Error> for NextError {
    fn from(value: sqlx::Error) -> Self {
        NextError::SqlError {
            source: value.into(),
        }
    }
}

impl IntoResponse for NextError {
    fn into_response(self) -> axum::response::Response {
        match self {
            NextError::SqlError { source } => source.into_response(),
            NextError::BadGateway { source } => {
                tracing::error!("Bad gateway: {}", Reporter(source));
                StatusCode::BAD_GATEWAY.into_response()
            }
            NextError::Forbidden => StatusCode::FORBIDDEN.into_response(),
            NextError::Conflict => StatusCode::CONFLICT.into_response(),
            NextError::NotFound => (StatusCode::NOT_FOUND, "Song not found").into_response(),
        }
    }
}

impl IntoResponses for NextError {
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
            (
                StatusCode::NOT_FOUND.as_str().to_string(),
                openapi::Response::new("Song not found: either a id was given of a song that is no in the queue, or the list is empty").into(),
            ),
        ]
        .into_iter()
        .chain(SqlError::responses())
        .collect()
    }
}

#[derive(serde::Deserialize, IntoParams)]
pub struct NextQueryParams {
    /// Force this call to be evaluated in the `auto-next` mode
    ///
    /// If missing, the call will pass in the `auto-next` mode only if the
    /// conditions for one to succeed are met
    #[serde(default)]
    pub auto: Option<bool>,
    /// Move to a particular song instead of the next in list
    #[serde(default)]
    pub song: Option<Uuid>,
}

/// Change the current song
///
/// Changes the current song to another one, and set the new one as playing.
///
/// If no song is given, the one on top of the queue will be used. If the queue
/// is empty or the given song is not in the queue, a `404 Not Found` error will
/// be returned.
///
/// This endpoint can be accessed through two possible channels. Either one has
/// the `QUEUE_NEXT` permission, or the `QUEUE_AUTO_NEXT` permission. The second
/// channel requires additional restrictions to be met:
/// - The current song must be null, or ended without stopping.
/// - The song cannot be specified, enabling only moving to the top song.
#[debug_handler(state = crate::App)]
#[utoipa::path(post, path = "/next",
responses(
    (status = StatusCode::NO_CONTENT, description = "Song changed"),
    NextError
),
params(NextQueryParams, QueuePathParams)
)]
#[instrument(name = "next", skip_all, fields(id = %id, user.id = %user.id()))]
pub async fn next(
    mut tx: SqlTx,
    collector: Collector<5>,
    client: ServicesClient,
    State(services): State<Arc<Services>>,
    Extension(user): Extension<Arc<QueueUser>>,
    Query(NextQueryParams { auto, song }): Query<NextQueryParams>,
    Path(QueuePathParams { id }): Path<QueuePathParams>,
) -> Result<(Changed, NoContent), NextError> {
    let auto = {
        let is_next_available = user.can(QueueUserAction::Song(QueueUserActionSong::Next));
        let is_auto_available = async {
            Ok::<_, NextError>(
                song.is_none()
                    && user.can(QueueUserAction::Song(QueueUserActionSong::AutoNext))
                    && {
                        let (current_song, current_song_start_at, now): (
                    Option<Uuid>,
                    Option<DateTime<FixedOffset>>,
                    DateTime<FixedOffset>,
                ) = sqlx::query_as(
                    "SELECT current_song, current_song_start_at, NOW() FROM queue WHERE id = $1",
                )
                .bind(id)
                .fetch_one(&mut tx)
                .await
                .map_err(SqlError::from)?;

                        if let Some(current_song) = current_song {
                            if let Some(current_song_start_at) = current_song_start_at {
                                // Get the duration of the current song
                                let current_song_duration = client
                                    .get(
                                        services
                                            .songs_url
                                            .join(&format!("songs/{current_song}"))
                                            .unwrap(),
                                    )
                                    .query(&SolvedQueryParams { source_data: false })
                                    .send()
                                    .await?
                                    .error_for_status()?
                                    .json::<Song>()
                                    .await?
                                    .duration;

                                // Auto-next is available if the current song is finished
                                current_song_start_at + current_song_duration <= now
                            } else {
                                // Current song is stopped, no auto-next
                                false
                            }
                        } else {
                            // No current song, auto-next is available
                            true
                        }
                    },
            )
        };
        if let Some(requested_auto) = auto {
            // User gave us a hint

            if requested_auto && is_auto_available.await? {
                // User asked for auto-next and it's available

                true
            } else if !requested_auto && is_next_available {
                // User asked for next and it's available

                false
            } else {
                // User asked for a method that's not available

                return Err(NextError::Forbidden);
            }
        } else if is_next_available {
            // User did not give us a hint, but next is available

            false
        } else if is_auto_available.await? {
            // User did not give us a hint, but auto-next is available

            true
        } else {
            // User did not give us a hint and has neither next nor auto-next
            return Err(NextError::Forbidden);
        }
    };

    tracing::debug!(
        auto,
        "Autorized trought {}",
        if auto { "auto-next" } else { "next" }
    );

    // Add the current song to the end of the queue
    let reenqueued = sqlx::query_as(unfill!(
        "
        WITH queue_data AS (
            SELECT
                queue.current_song,
                queue.current_song_queued_by
            FROM queue
            INNER JOIN queue_user 
                ON  queue_user.queue_id = queue.id
                AND queue_user.user_id  = queue.current_song_queued_by
            WHERE queue.id = $1
            AND queue.current_song IS NOT NULL
        ),
        update_queue AS (
            UPDATE queue
            SET
                current_song = NULL,
                current_song_start_at = NULL,
                current_song_position = NULL,
                current_song_queued_by = NULL
            WHERE id = $1
        )
        INSERT INTO queued_song (queue_id, song_id, queued_by)
        SELECT
            $1, current_song, current_song_queued_by
        FROM queue_data
        RETURNING song_id, queued_at
        "
    ))
    .bind(id)
    .fetch_optional(&mut tx)
    .await
    .map_err(SqlError::from)?;

    let mut event = PatchEventBuilder::queue(id);

    if let Some((song, queued_at)) = reenqueued {
        event = event
            .add(
                format!("/queue/{song}"),
                QueuedSong {
                    song: IdOrRep::Id(song),
                    queued_at,
                    likes: 0,
                    user_likes: 0,
                },
            )
            .move_("/current/song", format!("/queue/{song}/song"));
    }

    event = event.replace("/current", None::<Current>);

    // Remove the requested song from the queue
    let (song, queued_by): (Uuid, Uuid) = if let Some(song) = song {
        let queued_by = sqlx::query_scalar(unfill!(
            "
            DELETE FROM queued_song WHERE queue_id = $1 AND song_id = $2
            RETURNING queued_by
            "
        ))
        .bind(id)
        .bind(song)
        .fetch_optional(&mut tx)
        .await
        .map_err(SqlError::from)?;

        let Some(queued_by) = queued_by else {
            return Err(NextError::NotFound);
        };

        (song, queued_by)
    } else {
        // Delete and return top song

        let song = sqlx::query_as(
            unfill!(
                "
                WITH top_song_to_remove AS (
                    SELECT
                        qs.song_id
                    FROM
                        queued_song qs
                    LEFT JOIN
                        likes l ON qs.queue_id = l.queue_id AND qs.song_id = l.song_id
                    WHERE
                        qs.queue_id = $1
                    GROUP BY
                        qs.song_id, qs.queued_at
                    ORDER BY
                        COALESCE(SUM(l.count), 0) DESC,
                        qs.queued_at ASC
                    LIMIT 1
                )
                DELETE FROM
                    queued_song
                WHERE
                    queue_id = $1
                    AND song_id = (SELECT song_id FROM top_song_to_remove)
                RETURNING song_id, queued_by
                "
            )
            .trim_ascii(),
        )
        .bind(id)
        .fetch_optional(&mut tx)
        .await
        .map_err(SqlError::from)?;

        song.ok_or(NextError::NotFound)?
    };

    // Set the new current song
    let current_song_start_at = sqlx::query_scalar(unfill!(
        "
        UPDATE queue
        SET
            current_song = $2,
            current_song_start_at = NOW(),
            current_song_position = NULL,
            current_song_queued_by = $3
        WHERE id = $1
        RETURNING current_song_start_at
        "
    ))
    .bind(id)
    .bind(song)
    .bind(queued_by)
    .fetch_one(&mut tx)
    .await
    .map_err(SqlError::from)?;

    event
        .replace(
            "/current",
            Current::playing(IdOrRep::Id(song), current_song_start_at),
        )
        .move_(format!("/queue/{song}/song"), "/current/song")
        .remove(format!("/queue/{song}"))
        .build()
        .collect(&collector)
        .await;

    Ok((Changed::new(&mut tx, &collector, id).await?, NoContent))
}
