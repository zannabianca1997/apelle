use axum::{Router, extract::FromRef, routing::post};
use config::Config;
use snafu::{ResultExt as _, Snafu};
use sqlx::PgPool;

pub mod config;

mod providers;
mod sources;

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {
    ConnectionError { source: sqlx::Error },
}

#[derive(Debug, Clone, FromRef)]
pub struct App {
    db: PgPool,
    client: reqwest::Client,
}

pub async fn app(Config { db_url }: Config) -> Result<Router, MainError> {
    tracing::info!("Connecting to database");
    let db = PgPool::connect(db_url.as_str())
        .await
        .context(ConnectionSnafu)?;

    let client = reqwest::Client::new();

    Ok(Router::new()
        .route("/sources", post(sources::register))
        .route("/providers", post(providers::register))
        .with_state(App { db, client }))
}
