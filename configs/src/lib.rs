use axum::extract::FromRef;
use config::Config;
use futures::FutureExt as _;
use snafu::{ResultExt as _, Snafu};
use sqlx::PgPool;
use tracing::{Instrument as _, info_span};
use utoipa_axum::{router::OpenApiRouter, routes};

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

pub async fn app(Config { db_url }: Config) -> Result<OpenApiRouter, MainError> {
    tracing::info!("Connecting to database");

    let db = PgPool::connect(db_url.as_str())
        .map(|r| r.context(DbConnectionSnafu))
        .instrument(info_span!("Connecting to database"))
        .await?;

    Ok(OpenApiRouter::new()
        .routes(routes!(create::create))
        .routes(routes!(get::get, delete::delete))
        .nest("/public", OpenApiRouter::new().routes(routes!(get::get)))
        .with_state(App { db }))
}
