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
