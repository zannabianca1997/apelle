use axum::{Router, routing::get};
use config::Config;
use snafu::Snafu;

pub mod config;

mod config_processing;

mod create {
    use apelle_common::common_errors::SQLError;
    use apelle_configs_dtos::{QueueConfig, QueueConfigCreate};
    use axum::http::StatusCode;
    use snafu::Snafu;

    #[derive(Debug, Snafu)]
    pub enum CreateError {
        #[snafu(transparent)]
        SqlError { source: SQLError },
    }

    pub async fn create(
        config: QueueConfigCreate,
    ) -> Result<(StatusCode, QueueConfig), CreateError> {
        todo!()
    }
}

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {}

pub async fn app(config: Config) -> Result<Router, MainError> {
    Ok(Router::new().route(
        "/public",
        get(|| async { "Hello! The configs service is up and running." }),
    ))
}
