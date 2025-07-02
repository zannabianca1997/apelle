use std::{convert::Infallible, sync::Arc, time::Duration};

use apelle_common::{AuthHeaders, ServicesClient};
use axum::{
    debug_handler,
    extract::{Path, State},
    response::{
        IntoResponse, Sse,
        sse::{self, KeepAlive},
    },
};
use futures::{
    FutureExt, StreamExt as _,
    future::{Either, Ready},
};
use itertools::Itertools;
use reqwest::{Response, StatusCode};
use snafu::{ResultExt, Snafu};
use tokio::time::Instant;
use url::Url;
use utoipa::{IntoResponses, openapi};
use uuid::Uuid;

use crate::{
    QueuesService,
    config::SseConfig,
    events::{EventContent, PatchesLost, SubscribedClient},
};

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
    State(SseConfig {
        chunk_size,
        throttle,
        keep_alive,
    }): State<SseConfig>,
    user: AuthHeaders,
    client: ServicesClient,
    Path(id): Path<Uuid>,
    State(QueuesService {
        url: queues_url,
        sync_timeout,
    }): State<QueuesService>,
) -> Result<impl IntoResponse, GetEventsError> {
    // Start listening to events
    let events = subscriber.events(id, user.id());

    let state_machine = Arc::new(StateMachine {
        queues_url,
        id,
        client,
        sync_timeout,
    });

    // Ask the queues service to provide an initial state
    state_machine.clone().ask_sync_event().await?;

    // When some events are lost, ask for a sync event and wait it
    let events = events
        .scan(
            StreamState::DroppingUntilSync {
                timeout: Instant::now() + sync_timeout,
            },
            move |state, r| state_machine.run(state, r),
        )
        .filter_map(async |x| x);

    // Coalesce events that are sent in quick succession
    let events = events
        .ready_chunks(chunk_size)
        .flat_map(|chunk| tokio_stream::iter(chunk.into_iter().coalesce(EventContent::coalesce)));

    // Throttle the stream, to not send too many events in quick succession
    let events = tokio_stream::StreamExt::throttle(events, throttle);

    // Convert events to sse ones
    let events = events
        .map(|event| sse::Event::default().json_data(event).unwrap())
        .map(Ok::<_, Infallible>);

    // Create the sse, and optionally add a keep-alive message
    let mut sse = Sse::new(events);
    if let Some(keep_alive) = keep_alive {
        sse = sse.keep_alive(KeepAlive::new().interval(keep_alive));
    }

    Ok((
        // Disable nginx buffering
        [("X-Accel-Buffering", "no")],
        sse,
    ))
}

#[derive(Debug, Clone, Copy)]
enum StreamState {
    Running,
    // The stream is currently broken, dropping all events until the next sync
    DroppingUntilSync { timeout: Instant },

    Ended,
}

fn ready_left<R, T>(t: T) -> futures::future::Either<futures::future::Ready<T>, R> {
    futures::future::Either::Left(futures::future::ready(t))
}

struct StateMachine {
    queues_url: Url,
    id: Uuid,
    client: ServicesClient,
    sync_timeout: Duration,
}

impl StateMachine {
    async fn ask_sync_event(self: Arc<Self>) -> Result<(), GetEventsError> {
        self.client
            .post(
                self.queues_url
                    .join(&format!("/queues/{}/push_sync_event", self.id))
                    .unwrap(),
            )
            .send()
            .await
            .and_then(Response::error_for_status)
            .map(|_| ())
            .context(CannotReachQueuesServiceSnafu)
    }

    fn run(
        self: &Arc<Self>,
        state: &mut StreamState,
        r: Result<EventContent, PatchesLost>,
    ) -> Either<
        Ready<Option<Option<EventContent>>>,
        impl Future<Output = Option<Option<EventContent>>> + use<>,
    > {
        match (*state, r) {
            (StreamState::Running, Ok(event)) => {
                if let EventContent::Deleted = event {
                    *state = StreamState::Ended;
                }
                ready_left(Some(Some(event)))
            }
            (StreamState::Running, Err(PatchesLost { count })) => {
                tracing::warn!(count, "Lost patches, requesting a sync event");
                *state = StreamState::DroppingUntilSync {
                    timeout: Instant::now() + self.sync_timeout,
                };
                self.clone()
                    .ask_sync_event()
                    .map(|r| {
                        if let Ok(_) = r {
                            // Continue
                            Some(None)
                        } else {
                            // Request failed, drop the stream
                            None
                        }
                    })
                    .right_future()
            }
            (StreamState::DroppingUntilSync { timeout }, r) => {
                if timeout.elapsed() > self.sync_timeout {
                    // TODO: return error? For now we just drop the
                    // stream, prompting the client to reconnect
                    return ready_left(None);
                }

                match r {
                    Ok(event @ EventContent::Sync(_)) => {
                        *state = StreamState::Running;
                        return ready_left(Some(Some(event)));
                    }
                    Ok(_) => (),
                    Err(PatchesLost { count }) => {
                        tracing::warn!(
                            count,
                            "Lost even more patches as we were waiting for a sync"
                        )
                    }
                }
                ready_left(Some(None))
            }
            (StreamState::Ended, _) => ready_left(None),
        }
    }
}
