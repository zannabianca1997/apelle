use std::sync::Arc;

use apelle_common::{ServicesClient, db::SqlTx};
use apelle_queues_dtos::events::Collector;
use axum::{
    Extension, debug_handler,
    extract::{Path, Query, State},
    response::NoContent,
};
use tracing::instrument;

use crate::{
    QueuePathParams, QueuedSongPathParams, Services,
    handlers::next::{NextError, NextQueryParams, next},
    middleware::{
        etag::{Changed, HasIfMatch},
        user::QueueUser,
    },
};

/// Change the current song to this
///
/// Changes the current song to this song, and set it as playing
///
/// This endpoint need the `QUEUE_NEXT` permission.
#[debug_handler(state = crate::App)]
#[utoipa::path(post, path = "/next",
responses(
    (status = StatusCode::NO_CONTENT, description = "Song changed"),
    NextError
),
params(QueuedSongPathParams)
)]
#[instrument(name = "next", skip_all, fields(queue = %queue, song=%song, user.id = %user.id()))]
pub async fn next_song(
    tx: SqlTx,
    collector: Collector,
    client: ServicesClient,
    State(services): State<Arc<Services>>,
    Extension(user): Extension<Arc<QueueUser>>,
    has_if_match: Option<Extension<HasIfMatch>>,
    Path(QueuedSongPathParams { queue, song }): Path<QueuedSongPathParams>,
) -> Result<(Changed, NoContent), NextError> {
    return next(
        tx,
        collector,
        client,
        State(services),
        Extension(user),
        Query(NextQueryParams {
            auto: Some(false),
            song: Some(song),
        }),
        has_if_match,
        Path(QueuePathParams { id: queue }),
    )
    .await;
}
