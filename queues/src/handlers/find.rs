use apelle_common::db::{SqlError, SqlTx};
use axum::{Json, debug_handler, extract::Query, response::IntoResponse};
use reqwest::StatusCode;
use snafu::Snafu;
use utoipa::{IntoParams, IntoResponses, openapi};
use uuid::Uuid;

#[derive(Debug, Snafu)]
pub enum FindError {
    #[snafu(transparent)]
    SqlError {
        source: SqlError,
    },
    Forbidden,
    NotFound,
}

impl IntoResponse for FindError {
    fn into_response(self) -> axum::response::Response {
        match self {
            FindError::SqlError { source } => source.into_response(),
            FindError::Forbidden => StatusCode::FORBIDDEN.into_response(),
            FindError::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

impl IntoResponses for FindError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [
            (
                StatusCode::FORBIDDEN.as_str().to_string(),
                openapi::Response::new("User is not allowed to read the queue data").into(),
            ),
            (
                StatusCode::NOT_FOUND.as_str().to_string(),
                openapi::Response::new("Queue not found").into(),
            ),
        ]
        .into_iter()
        .chain(SqlError::responses())
        .collect()
    }
}

#[derive(serde::Deserialize, IntoParams)]
pub struct FindQueryParams {
    /// The code of the searched queue
    pub code: String,
}

/// Read the queue data
#[debug_handler(state = crate::App)]
#[utoipa::path(get, path = "/",
    responses(
        (status = StatusCode::OK, description = "Queue found", 
         content_type = "text/json", body = Uuid),
        FindError
    ),
    params(FindQueryParams)
)]
pub async fn find(
    mut tx: SqlTx,
    Query(FindQueryParams { code }): Query<FindQueryParams>,
) -> Result<Json<Uuid>, FindError> {
    let id: Uuid = sqlx::query_scalar("SELECT id FROM queue WHERE code = $1")
        .bind(code)
        .fetch_optional(&mut tx)
        .await
        .map_err(SqlError::from)?
        .ok_or(FindError::NotFound)?;

    Ok(Json(id))
}
