use axum::Router;
use config::Config;
use snafu::Snafu;

pub mod config;

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {}

pub fn app(config: Config) -> Result<Router, MainError> {
    Ok(Router::new())
}
