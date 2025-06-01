use axum::{http::StatusCode, response::IntoResponse};
use snafu::Snafu;

use crate::Reporter;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub struct SQLError {
    pub source: sqlx::Error,
}

impl IntoResponse for SQLError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("SQL Error: {}", Reporter(self));
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub struct CacheError {
    pub source: redis::RedisError,
}

impl IntoResponse for CacheError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Cache Error: {}", Reporter(self));
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub struct PubSubError {
    pub source: redis::RedisError,
}

impl IntoResponse for PubSubError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("PubSub Error: {}", Reporter(self));
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
