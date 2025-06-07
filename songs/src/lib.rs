use axum::{
    Router,
    extract::FromRef,
    routing::{get, post},
};
use config::Config;
use futures::FutureExt;
use redis::IntoConnectionInfo;
use snafu::{ResultExt as _, Snafu};
use sqlx::PgPool;
use tracing::{Instrument, info_span};

const CACHE_NAMESPACE: &str = "apelle:songs:";

pub mod config;

mod providers;
mod resolve;
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
}

pub async fn app(
    Config {
        db_url,
        cache_url,
        honor_fast_handshake,
        seen_sources_queue_size,
    }: Config,
) -> Result<Router, MainError> {
    tracing::info!("Connecting to database and cache");

    let db = PgPool::connect(db_url.as_str())
        .map(|r| r.context(DbConnectionSnafu))
        .instrument(info_span!("Connecting to database"));

    let mut conn_info = cache_url
        .into_connection_info()
        .context(CacheConnectionSnafu)?;
    if conn_info.redis.protocol != redis::ProtocolVersion::RESP3 {
        tracing::warn!(
            proposed_protocol =? conn_info.redis.protocol,
            "Apelle only supports RESP3 protol, switching to it"
        );
        conn_info.redis.protocol = redis::ProtocolVersion::RESP3;
    }
    let client = redis::Client::open(conn_info).context(CacheConnectionSnafu)?;
    let cache = client
        .get_connection_manager()
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
                .route("/resolve", post(resolve::resolve))
                .route("/solved/{id}", get(solved::get).delete(solved::delete)),
        )
        .with_state(App {
            db,
            client,
            cache,
            providers_config: ProvidersConfig {
                honor_fast_handshake,
            },
            seen_sources,
        }))
}
