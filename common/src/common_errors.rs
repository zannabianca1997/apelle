use axum::{http::StatusCode, response::IntoResponse};
use snafu::Snafu;
use utoipa::{IntoResponses, openapi};

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

impl IntoResponses for SQLError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [(
            StatusCode::INTERNAL_SERVER_ERROR.as_str().to_owned(),
            openapi::RefOr::T(openapi::Response::new("Internal Server Error")),
        )]
        .into_iter()
        .collect()
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

impl IntoResponses for CacheError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [(
            StatusCode::INTERNAL_SERVER_ERROR.as_str().to_owned(),
            openapi::RefOr::T(openapi::Response::new("Internal Server Error")),
        )]
        .into_iter()
        .collect()
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

impl IntoResponses for PubSubError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [(
            StatusCode::INTERNAL_SERVER_ERROR.as_str().to_owned(),
            openapi::RefOr::T(openapi::Response::new("Internal Server Error")),
        )]
        .into_iter()
        .collect()
    }
}
