use axum::{Router, routing::get};
use config::Config;
use snafu::Snafu;

pub mod config;

mod config_processing;

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {}

pub async fn app(config: Config) -> Result<Router, MainError> {
    Ok(Router::new().route(
        "/public",
        get(|| async { "Hello! The configs service is up and running." }),
    ))
}
