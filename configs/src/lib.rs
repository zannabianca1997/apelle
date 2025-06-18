use axum::{
    Router,
    extract::FromRef,
    routing::{get, post},
};
use config::Config;
use futures::FutureExt as _;
use snafu::{ResultExt as _, Snafu};
use sqlx::PgPool;
use tracing::{Instrument as _, info_span};

pub mod config;

mod config_processing;

mod create;
mod delete;
mod get;

#[derive(Clone, FromRef)]
pub struct App {
    db: PgPool,
}

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {
    DbConnectionError { source: sqlx::Error },
}

pub async fn app(Config { db_url }: Config) -> Result<Router, MainError> {
    tracing::info!("Connecting to database");

    let db = PgPool::connect(db_url.as_str())
        .map(|r| r.context(DbConnectionSnafu))
        .instrument(info_span!("Connecting to database"))
        .await?;

    Ok(Router::new()
        .route("/queues", post(create::create))
        .route("/queues/{id}", get(get::get).delete(delete::delete))
        .nest(
            "/public",
            Router::new().route("/queues/{id}", get(get::get)),
        )
        .with_state(App { db }))
}
