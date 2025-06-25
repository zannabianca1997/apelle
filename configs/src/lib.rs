use apelle_common::db::{SqlState, db_state_and_layer};
use axum::extract::FromRef;
use config::Config;
use snafu::{ResultExt as _, Snafu};
use utoipa_axum::{router::OpenApiRouter, routes};

pub mod config;

mod config_processing;

mod create;
mod delete;
mod get;

#[derive(Clone, FromRef)]
pub struct App {
    db: SqlState,
}

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {
    DbConnectionError { source: sqlx::Error },
}

pub async fn app(Config { db_url }: Config) -> Result<OpenApiRouter, MainError> {
    tracing::info!("Connecting to database");

    let (db, sql_layer) = db_state_and_layer(db_url)
        .await
        .context(DbConnectionSnafu)?;

    Ok(OpenApiRouter::new()
        .routes(routes!(create::create))
        .routes(routes!(get::get, delete::delete))
        .nest("/public", OpenApiRouter::new().routes(routes!(get::get)))
        .route_layer(sql_layer)
        .with_state(App { db }))
}
