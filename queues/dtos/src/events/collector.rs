use std::{mem, sync::Arc};

use apelle_common::common_errors::PubSubError;
use arrayvec::ArrayVec;
use axum::{
    Extension,
    extract::{FromRequestParts, OptionalFromRequestParts, Request, State},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use tokio::sync::Mutex;

use super::{Event, Publisher};

#[derive(Debug, Clone)]
pub struct Collector<const CAP: usize> {
    inner: Arc<Mutex<ArrayVec<Event, CAP>>>,
}

impl<const CAP: usize> Collector<CAP> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(ArrayVec::new())),
        }
    }

    /// Push an event into the collector
    pub async fn collect(&self, event: Event) {
        self.inner.lock().await.push(event);
    }

    /// Publish all events in the collector
    pub async fn publish(self, publisher: &mut Publisher) -> Result<(), PubSubError> {
        let events = match Arc::try_unwrap(self.inner) {
            Ok(events) => events.into_inner(),
            Err(inner) => {
                tracing::warn!(
                    "References to the collector were present when `publish` was called"
                );
                let mut events = inner.lock().await;
                mem::take(&mut *events)
            }
        };
        if events.is_empty() {
            return Ok(());
        }
        publisher.publish_all(events).await
    }
}

pub async fn event_middleware<const CAP: usize>(
    State(mut publisher): State<Publisher>,
    mut request: Request,
    next: Next,
) -> Result<Response, PubSubError> {
    let collector = Collector::<CAP>::new();
    request.extensions_mut().insert(collector.clone());

    // Run the next middleware
    let response = next.run(request).await;

    // If the response is a client error or server error, we don't want to
    // publish anything
    if response.status().is_client_error() || response.status().is_server_error() {
        return Ok(response);
    }

    // publish the events
    collector.publish(&mut publisher).await?;

    Ok(response)
}

impl<S, const CAP: usize> FromRequestParts<S> for Collector<CAP>
where
    S: Sync,
    Extension<Self>: FromRequestParts<S>,
{
    // TODO: customize this rejection
    type Rejection = <Extension<Self> as FromRequestParts<S>>::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        <Extension<Self> as FromRequestParts<S>>::from_request_parts(parts, state)
            .await
            .map(|x| x.0)
    }
}

impl<S, const CAP: usize> OptionalFromRequestParts<S> for Collector<CAP>
where
    S: Sync,
    Extension<Self>: OptionalFromRequestParts<S>,
{
    type Rejection = <Extension<Self> as OptionalFromRequestParts<S>>::Rejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        <Extension<Self> as OptionalFromRequestParts<S>>::from_request_parts(parts, state)
            .await
            .map(|x| x.map(|x| x.0))
    }
}

impl Event {
    pub async fn collect(self, collector: &Collector<5>) -> () {
        collector.collect(self).await
    }
}
