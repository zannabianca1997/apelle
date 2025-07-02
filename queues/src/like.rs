use std::{iter::once, sync::Arc};

use apelle_common::{
    common_errors::PubSubError,
    db::{SqlError, SqlTx},
};
use apelle_configs_dtos::{QueueUserAction, QueueUserActionSong};
use apelle_queues_events::events::{BuildPatchEvent as _, Event, PatchEventBuilder, Publisher};
use axum::{
    Extension, debug_handler,
    extract::{Path, State},
    response::{IntoResponse, NoContent},
};
use reqwest::StatusCode;
use snafu::Snafu;
use textwrap_macros::unfill;
use utoipa::{IntoResponses, openapi};
use uuid::Uuid;

use crate::{QueuedSongPathParams, middleware::user::QueueUser};

#[derive(Debug, Snafu)]
pub enum LikeError {
    #[snafu(transparent)]
    SqlError {
        source: SqlError,
    },
    Forbidden,

    #[snafu(transparent)]
    PubSub {
        source: PubSubError,
    },
}

impl From<sqlx::Error> for LikeError {
    fn from(value: sqlx::Error) -> Self {
        LikeError::SqlError {
            source: value.into(),
        }
    }
}

impl IntoResponse for LikeError {
    fn into_response(self) -> axum::response::Response {
        match self {
            LikeError::SqlError { source } => source.into_response(),
            LikeError::Forbidden => StatusCode::FORBIDDEN.into_response(),
            LikeError::PubSub { source } => source.into_response(),
        }
    }
}

impl IntoResponses for LikeError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [(
            StatusCode::FORBIDDEN.as_str().to_string(),
            openapi::Response::new("User is not allowed to like songs").into(),
        )]
        .into_iter()
        .chain(SqlError::responses())
        .chain(PubSubError::responses())
        .collect()
    }
}

/// Read the queue data
#[debug_handler(state = crate::App)]
#[utoipa::path(post, path = "/like",
responses(
    (status = StatusCode::OK, description = "Song liked"),
    LikeError
),
params(QueuedSongPathParams)
)]
pub async fn like(
    mut tx: SqlTx,
    State(mut publisher): State<Publisher>,
    Extension(user): Extension<Arc<QueueUser>>,
    Path(QueuedSongPathParams { queue, song }): Path<QueuedSongPathParams>,
) -> Result<NoContent, LikeError> {
    if !user.can(QueueUserAction::Song(QueueUserActionSong::Like)) || user.role().max_likes == 0 {
        return Err(LikeError::Forbidden);
    }

    // Remove a like if the user reached his maximum
    let deleted_from = if user.role().max_likes <= user.likes() {
        let deleted: Option<Uuid> =
            sqlx::query_scalar(unfill!("SELECT remove_oldest_like($1, $2, $3)"))
                .bind(queue)
                .bind(user.id())
                .bind(song)
                .fetch_one(&mut tx)
                .await?;

        // If no like was removed, that means that the user liked only this
        // song. Adding and removing a like would be a no-op
        let Some(deleted) = deleted else {
            return Ok(NoContent);
        };

        // Build the dislike events
        Some(deleted)
    } else {
        None
    };

    // Add a like
    let _ = sqlx::query(unfill!(
        "
        INSERT INTO likes (queue_id, song_id, user_id)
        VALUES ($1, $2, $3)
        ON CONFLICT (queue_id, song_id, user_id, given_at) DO UPDATE
        SET count = likes.count + 1;
        "
    ))
    .bind(queue)
    .bind(song)
    .bind(user.id())
    .execute(&mut tx)
    .await?;

    // Update the queue id, as the song might now be the first one and influence
    // the next id
    let player_state_id: Uuid = sqlx::query_scalar(
        unfill!(
            "
                UPDATE queue
                SET player_state_id = gen_random_uuid()
                WHERE id = $1
                RETURNING player_state_id
                "
        )
        .trim_ascii(),
    )
    .bind(queue)
    .fetch_one(&mut tx)
    .await?;

    let mut queue_event = Event::queue(queue).replace("/player_state_id", player_state_id);
    let mut user_event = PatchEventBuilder::user(queue, user.id());

    // Publish the new like count
    for changed_song in deleted_from.into_iter().chain(once(song)) {
        let (likes, user_likes): (i16, i16) = sqlx::query_as(
            unfill!(
                "
                SELECT
                    COALESCE(tl.likes_count, 0::smallint) AS likes,
                    COALESCE(ul.user_likes_count, 0::smallint) AS user_likes
                FROM
                    queued_song qs
                LEFT JOIN LATERAL (
                    SELECT
                        l.queue_id,
                        l.song_id,
                        SUM(l.count)::smallint AS likes_count
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
                        SUM(l.count)::smallint AS user_likes_count
                    FROM
                        likes l
                    WHERE
                        l.queue_id = qs.queue_id
                        AND l.song_id = qs.song_id
                        AND l.user_id = $3
                    GROUP BY
                        l.queue_id,
                        l.song_id
                ) ul ON TRUE
                WHERE
                    qs.queue_id = $1 AND qs.song_id = $2
                "
            )
            .trim_ascii(),
        )
        .bind(queue)
        .bind(changed_song)
        .bind(user.id())
        .fetch_one(&mut tx)
        .await?;

        queue_event = queue_event.replace(format!("/queue/{changed_song}/likes"), likes as u16);
        user_event = user_event.replace(
            format!("/queue/{changed_song}/user_likes"),
            user_likes as u16,
        );
    }

    publisher.publish(queue_event.build()).await?;
    publisher.publish(user_event.build()).await?;

    Ok(NoContent)
}
