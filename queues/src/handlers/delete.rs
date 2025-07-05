use std::sync::Arc;

use apelle_common::db::{SqlError, SqlTx};
use apelle_configs_dtos::{QueueUserAction, QueueUserActionQueue};
use apelle_queues_events::events::{Collector, Event};
use axum::{
    Extension, debug_handler,
    extract::Path,
    response::{IntoResponse, NoContent},
};
use futures::{StreamExt, TryStreamExt as _};
use reqwest::StatusCode;
use snafu::Snafu;
use utoipa::{IntoResponses, openapi};

use crate::{QueuePathParams, middleware::user::QueueUser};

#[derive(Debug, Snafu)]
pub enum DeleteError {
    #[snafu(transparent)]
    SqlError {
        source: SqlError,
    },
    Forbidden,
}

impl IntoResponse for DeleteError {
    fn into_response(self) -> axum::response::Response {
        match self {
            DeleteError::SqlError { source } => source.into_response(),
            DeleteError::Forbidden => StatusCode::FORBIDDEN.into_response(),
        }
    }
}

impl IntoResponses for DeleteError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [(
            StatusCode::FORBIDDEN.as_str().to_string(),
            openapi::Response::new("User is not allowed to delete the queue").into(),
        )]
        .into_iter()
        .chain(SqlError::responses())
        .collect()
    }
}

/// Read the queue data
#[debug_handler(state = crate::App)]
#[utoipa::path(delete, path = "/",
    responses(
        (status = StatusCode::NO_CONTENT, description = "Queue deleted"),
        DeleteError
    ),
    params(QueuePathParams)
)]
pub async fn delete(
    mut tx: SqlTx,
    collector: Collector<5>,
    Extension(user): Extension<Arc<QueueUser>>,
    Path(QueuePathParams { id }): Path<QueuePathParams>,
) -> Result<NoContent, DeleteError> {
    if !user.can(QueueUserAction::Queue(QueueUserActionQueue::Delete)) {
        return Err(DeleteError::Forbidden);
    }

    sqlx::query("DELETE FROM queue WHERE id = $1")
        .bind(id)
        .execute(&mut tx)
        .await
        .map_err(SqlError::from)?;

    Event::queue(id).deleted().collect(&collector).await;

    Ok(NoContent)
}
