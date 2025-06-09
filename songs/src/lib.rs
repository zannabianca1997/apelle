use apelle_common::cache_pubsub;
use axum::{
    Router,
    extract::FromRef,
    routing::{get, post},
};
use chrono::Duration;
use config::Config;
use futures::FutureExt;
use snafu::{ResultExt as _, Snafu};
use sqlx::PgPool;
use tracing::{Instrument, info_span};

const CACHE_NAMESPACE: &str = "apelle:songs:";

pub mod config;

mod providers;
mod resolve;
mod search;
mod seen_sources;
mod solved;
mod sources;

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {
    DbConnectionError { source: sqlx::Error },
    CacheConnectionError { source: redis::RedisError },
}

#[derive(Clone, FromRef)]
pub struct App {
    db: PgPool,
    cache: redis::aio::ConnectionManager,
    client: reqwest::Client,
    providers_config: ProvidersConfig,
    seen_sources: seen_sources::SeenSourcesWorker,
}

#[derive(Debug, Clone, Copy)]
pub struct ProvidersConfig {
    pub honor_fast_handshake: bool,
    pub cache_expiration: Duration,
    pub page_size: u32,
}

pub async fn app(
    Config {
        db_url,
        cache_url,
        honor_fast_handshake,
        seen_sources_queue_size,
        cache_expiration,
        page_size,
    }: Config,
) -> Result<Router, MainError> {
    tracing::info!("Connecting to database and cache");

    let db = PgPool::connect(db_url.as_str())
        .map(|r| r.context(DbConnectionSnafu))
        .instrument(info_span!("Connecting to database"));

    let cache = cache_pubsub::connect(cache_url)
        .map(|r| r.context(CacheConnectionSnafu))
        .instrument(info_span!("Connecting to cache"));

    let (db, cache) = tokio::try_join!(db, cache)?;

    let client = reqwest::Client::new();

    let seen_sources =
        seen_sources::SeenSourcesWorker::new(db.clone(), seen_sources_queue_size).await;

    Ok(Router::new()
        .route("/sources", get(sources::list).post(sources::register))
        .route("/providers", post(providers::register))
        .nest(
            "/public",
            Router::new()
                .route("/sources", get(sources::list))
                .route("/search", get(search::search))
                .route("/resolve", post(resolve::resolve))
                .route("/solved/{id}", get(solved::get).delete(solved::delete)),
        )
        .with_state(App {
            db,
            client,
            cache,
            providers_config: ProvidersConfig {
                honor_fast_handshake,
                cache_expiration,
                page_size,
            },
            seen_sources,
        }))
}
