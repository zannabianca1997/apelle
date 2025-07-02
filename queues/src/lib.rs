use std::sync::Arc;

use apelle_common::{
    cache_pubsub,
    db::{SqlState, db_state_and_layer},
};
use axum::{extract::FromRef, middleware::from_fn_with_state};
use config::Config;
use futures::FutureExt as _;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt as _, Snafu};
use tracing::{Instrument as _, info_span};
use url::Url;
use utoipa::{IntoParams, OpenApi};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    config::CodeConfig,
    middleware::{config::extract_queue_config, user::extract_queue_user},
};

pub mod config;

mod dtos;
mod model;

mod middleware {
    pub mod config;
    pub mod user;
}

mod create;
mod enqueue;
mod events;
mod get;
mod push_sync_event;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, IntoParams)]
struct QueuePathParams {
    pub id: Uuid,
}

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {
    DbConnectionError {
        source: sqlx::Error,
    },
    CacheConnectionError {
        source: redis::RedisError,
    },
    #[snafu(display("Code alphabet must not be empty"))]
    EmptyCodeAlphabet,
}

#[derive(Clone, FromRef)]
pub struct App {
    db: SqlState,
    cache: redis::aio::ConnectionManager,
    client: reqwest::Client,
    services: Arc<Services>,
    code: Arc<CodeConfig>,
}

#[derive(Clone)]
pub struct Services {
    /// Url of the `songs` service
    pub songs_url: Url,
    /// Url of the `configs` service
    pub configs_url: Url,
}

#[derive(OpenApi)]
struct AppApi;

pub async fn app(
    Config {
        db_url,
        cache_url,
        songs_url,
        configs_url,
        code,
    }: Config,
) -> Result<OpenApiRouter, MainError> {
    if code.alphabet.is_empty() {
        return Err(MainError::EmptyCodeAlphabet);
    }

    tracing::info!("Connecting to database and cache");

    let db = db_state_and_layer(db_url)
        .map(|r| r.context(DbConnectionSnafu))
        .instrument(info_span!("Connecting to database"));

    let cache = cache_pubsub::connect(cache_url)
        .map(|r| r.context(CacheConnectionSnafu))
        .instrument(info_span!("Connecting to cache"));

    let ((db, tx_layer), cache) = tokio::try_join!(db, cache)?;

    let client = reqwest::Client::new();

    let app = App {
        db,
        cache,
        client,
        services: Arc::new(Services {
            songs_url,
            configs_url,
        }),
        code: Arc::new(code),
    };

    let middleware = tower::ServiceBuilder::new()
        .layer(from_fn_with_state(app.clone(), extract_queue_config))
        .layer(from_fn_with_state(app.clone(), extract_queue_user));

    Ok(OpenApiRouter::with_openapi(AppApi::openapi())
        .nest(
            "/queues/{id}",
            OpenApiRouter::new()
                .routes(routes!(push_sync_event::push_sync_event))
                .route_layer(middleware.clone()),
        )
        .nest(
            "/public",
            OpenApiRouter::new().routes(routes!(create::create)).nest(
                "/{id}",
                OpenApiRouter::new()
                    .routes(routes!(get::get))
                    .routes(routes!(events::events))
                    .routes(routes!(enqueue::enqueue))
                    .route_layer(middleware),
            ),
        )
        .route_layer(tx_layer)
        .with_state(app))
}
