use apelle_common::db::{SqlError, SqlTx};
use axum::{
    debug_handler,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, NoContent},
};
use snafu::Snafu;
use utoipa::{
    IntoResponses,
    openapi::{self, RefOr},
};
use uuid::Uuid;

#[derive(Debug, Snafu)]
pub enum DeleteError {
    #[snafu(transparent)]
    SqlError {
        source: SqlError,
    },
    NotFound,
    CannotDeleteDefaultConfig,
}

impl IntoResponse for DeleteError {
    fn into_response(self) -> axum::response::Response {
        match self {
            DeleteError::SqlError { source } => source.into_response(),
            DeleteError::NotFound => StatusCode::NOT_FOUND.into_response(),
            DeleteError::CannotDeleteDefaultConfig => StatusCode::FORBIDDEN.into_response(),
        }
    }
}

impl IntoResponses for DeleteError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [
            (
                StatusCode::NOT_FOUND.as_str().to_string(),
                RefOr::T(openapi::Response::new("Queue config not found")),
            ),
            (
                StatusCode::FORBIDDEN.as_str().to_string(),
                RefOr::T(openapi::Response::new(
                    "Cannot delete default config (nil UUID)",
                )),
            ),
        ]
        .into_iter()
        .chain(SqlError::responses())
        .collect()
    }
}

#[debug_handler(state=crate::App)]
#[utoipa::path(delete, path = "/queues/{id}", responses((status = StatusCode::OK, description = "Queue config deleted"), DeleteError))]
/// Delete a queue config
pub async fn delete(Path(id): Path<Uuid>, mut tx: SqlTx) -> Result<NoContent, DeleteError> {
    if id.is_nil() {
        return Err(DeleteError::CannotDeleteDefaultConfig);
    }
    let rows = sqlx::query("DELETE FROM queue_config WHERE id = $1")
        .bind(id)
        .execute(&mut tx)
        .await
        .map_err(SqlError::from)?
        .rows_affected();
    if rows == 0 {
        return Err(DeleteError::NotFound);
    }
    Ok(NoContent)
}
