use std::convert::Infallible;

use apelle_common::{AuthHeaders, ServicesClient};
use axum::{
    debug_handler,
    extract::{Path, State},
    response::{IntoResponse, Sse, sse},
};
use futures::TryStreamExt as _;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
use tokio_stream::StreamExt;
use utoipa::{IntoResponses, openapi};
use uuid::Uuid;

use crate::{QueuesUrl, events::SubscribedClient};

/// Errors happening starting the event stream
#[derive(Debug, Snafu)]
pub enum GetEventsError {
    CannotReachQueuesService { source: reqwest::Error },
}

impl IntoResponse for GetEventsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            GetEventsError::CannotReachQueuesService { source } => {
                tracing::error!(%source, "Cannot reach the queues service");
                StatusCode::BAD_GATEWAY.into_response()
            }
        }
    }
}

impl IntoResponses for GetEventsError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [(
            StatusCode::BAD_GATEWAY.as_str().to_string(),
            openapi::Response::new("Cannot reach the queues service").into(),
        )]
        .into_iter()
        .collect()
    }
}

#[debug_handler(state=crate::App)]
#[utoipa::path(get, path = "/events/{id}", 
    responses((status = StatusCode::OK, content_type = "text/event-stream")))]
pub async fn events(
    State(subscriber): State<SubscribedClient>,
    user: AuthHeaders,
    client: ServicesClient,
    Path(id): Path<Uuid>,
    State(QueuesUrl(queues_url)): State<QueuesUrl>,
) -> Result<impl IntoResponse, GetEventsError> {
    // Start listening to events
    let events = subscriber.events(id, user.id());

    // Ask the queues service to provide a sync event
    client
        .post(
            queues_url
                .join(&format!("/queues/{id}/push_sync_event"))
                .unwrap(),
        )
        .send()
        .await
        .context(CannotReachQueuesServiceSnafu)?;

    Ok((
        // Disable nginx buffering
        [("X-Accel-Buffering", "no")],
        Sse::new(
            events
                .map_ok(|event| sse::Event::default().json_data(event).unwrap())
                // Close on error, prompting client to reconnect
                .map_while(|r| r.ok())
                .map(Ok::<_, Infallible>),
        ),
    ))
}
