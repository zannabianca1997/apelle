use apelle_common::common_errors::PubSubError;
use axum::extract::FromRef;
use redis::Client;
use snafu::{ResultExt, Snafu};
use url::Url;
use utoipa::OpenApi;

mod config;
pub mod events;

use config::Config;
use utoipa_axum::router::OpenApiRouter;

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
    queues_url: QueuesUrl,
}

#[derive(Clone)]
struct QueuesUrl(Url);

#[derive(OpenApi)]
struct AppApi;

pub async fn app(
    Config {
        pubsub_url,
        queues_url,
    }: Config,
) -> Result<OpenApiRouter, MainError> {
    tracing::info!("Connecting to pubsub");
    let subscriber = events::SubscribedClient::new(
        Client::open(pubsub_url).context(CacheConnectionSnafu)?,
        Default::default(),
    )
    .await?;

    Ok(
        OpenApiRouter::with_openapi(AppApi::openapi()).with_state(App {
            client: reqwest::Client::new(),
            subscriber,
            queues_url: QueuesUrl(queues_url),
        }),
    )
}
