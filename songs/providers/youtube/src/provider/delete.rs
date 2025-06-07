use super::SongsPathParams;
use apelle_common::common_errors::SQLError;
use axum::{
    debug_handler,
    extract::Path,
    response::{IntoResponse, NoContent},
};
use reqwest::StatusCode;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum DeleteError {
    #[snafu(transparent)]
    SQLError {
        source: SQLError,
    },
    NotFound,
}

impl IntoResponse for DeleteError {
    fn into_response(self) -> axum::response::Response {
        match self {
            DeleteError::SQLError { source } => source.into_response(),
            DeleteError::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

#[debug_handler(state=crate::App)]
pub async fn delete(Path(SongsPathParams { id }): Path<SongsPathParams>) -> NoContent {
    tracing::info!(%id, "Deleting song");

    // Song was already deleted thanks to the ON DELETE CASCADE constraints

    NoContent
}
