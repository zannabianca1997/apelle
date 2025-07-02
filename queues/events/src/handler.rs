use std::collections::BTreeMap;

use apelle_common::AuthHeaders;
use axum::{
    debug_handler,
    extract::{Path, State},
    response::{IntoResponse, Sse, sse},
};
use futures::TryStreamExt as _;
use snafu::Snafu;
use tokio_stream::StreamExt;
use utoipa::IntoResponses;
use uuid::Uuid;

use crate::{QueuesUrl, events::SubscribedClient};

#[derive(Debug, Snafu)]
pub enum EventsError {}

impl IntoResponse for EventsError {
    fn into_response(self) -> axum::response::Response {
        match self {}
    }
}

impl IntoResponses for EventsError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        BTreeMap::new()
    }
}

#[debug_handler(state=crate::App)]
#[utoipa::path(get, path = "/events/{id}", 
    responses((status = StatusCode::OK, content_type = "text/event-stream")))]
pub async fn events(
    State(subscriber): State<SubscribedClient>,
    user: AuthHeaders,
    Path(id): Path<Uuid>,
    State(QueuesUrl(queues_url)): State<QueuesUrl>,
) -> impl IntoResponse {
    (
        // Disable nginx buffering
        [("X-Accel-Buffering", "no")],
        Sse::new(
            subscriber
                .events(id, user.id())
                .map_ok(patch_to_sse_event)
                // Close on error, prompting client to reconnect
                .map_while(|r| r.ok())
                .map(Ok::<sse::Event, EventsError>),
        ),
    )
}

pub fn patch_to_sse_event(event: json_patch::Patch) -> sse::Event {
    sse::Event::default().json_data(event).unwrap()
}
