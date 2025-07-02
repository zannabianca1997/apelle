use std::time::Duration;

use apelle_common::common_errors::PubSubError;
use axum::extract::FromRef;
use redis::Client;
use serde::Deserialize;
use snafu::{ResultExt, Snafu};
use url::Url;
use utoipa::OpenApi;

mod config;
pub mod events;
mod handler;

use config::Config;
use utoipa_axum::{router::OpenApiRouter, routes};

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {
    CacheConnectionError {
        source: redis::RedisError,
    },
    #[snafu(transparent)]
    PubSub {
        source: PubSubError,
    },
}

#[derive(Clone, FromRef)]
struct App {
    client: reqwest::Client,
    subscriber: events::SubscribedClient,
    queues: QueuesService,
}

#[derive(Clone, Deserialize, Debug)]
pub struct QueuesService {
    url: Url,
    #[serde(with = "apelle_common::iso8601::duration")]
    sync_timeout: Duration,
}

#[derive(OpenApi)]
struct AppApi;

pub async fn app(
    Config {
        pubsub_url,
        queues,
        inner_queue_capacity,
    }: Config,
) -> Result<OpenApiRouter, MainError> {
    tracing::info!("Connecting to pubsub");
    let subscriber = events::SubscribedClient::new(
        Client::open(pubsub_url).context(CacheConnectionSnafu)?,
        events::SubscribedClientConfig {
            capacity: inner_queue_capacity,
            pub_sub: Default::default(),
        },
    )
    .await?;

    Ok(OpenApiRouter::with_openapi(AppApi::openapi())
        .routes(routes!(handler::events))
        .with_state(App {
            client: reqwest::Client::new(),
            subscriber,
            queues,
        }))
}
