use std::sync::Arc;

use apelle_common::cache_pubsub;
use axum::extract::FromRef;
use config::Config;
use futures::FutureExt as _;
use snafu::{ResultExt as _, Snafu};
use sqlx::PgPool;
use tracing::{Instrument as _, info_span};
use url::Url;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::config::CodeConfig;

pub mod config;

mod dtos;
mod model;

mod create;

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
    db: PgPool,
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

    let db = PgPool::connect(db_url.as_str())
        .map(|r| r.context(DbConnectionSnafu))
        .instrument(info_span!("Connecting to database"));

    let cache = cache_pubsub::connect(cache_url)
        .map(|r| r.context(CacheConnectionSnafu))
        .instrument(info_span!("Connecting to cache"));

    let (db, cache) = tokio::try_join!(db, cache)?;

    let client = reqwest::Client::new();

    Ok(OpenApiRouter::new()
        .nest(
            "/public",
            OpenApiRouter::new().routes(routes!(create::create)),
        )
        .with_state(App {
            db,
            cache,
            client,
            services: Arc::new(Services {
                songs_url,
                configs_url,
            }),
            code: Arc::new(code),
        }))
}
