use std::sync::Arc;

use apelle_common::{
    Reporter, ServicesClient,
    common_errors::{SQLError, SQLSnafu},
};
use apelle_configs_dtos::QueueConfig;
use axum::{
    debug_middleware,
    extract::{Path, Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use snafu::{OptionExt, ResultExt, Snafu};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{QueuePathParams, Services};

#[derive(Debug, Snafu)]
pub enum FetchConfigError {
    #[snafu(transparent)]
    SqlError {
        source: SQLError,
    },
    #[snafu(transparent)]
    ConnectionError {
        source: reqwest::Error,
    },

    QueueNotFound,
}

impl IntoResponse for FetchConfigError {
    fn into_response(self) -> axum::response::Response {
        match self {
            FetchConfigError::SqlError { source } => source.into_response(),
            FetchConfigError::ConnectionError { source } => {
                tracing::error!("Connection error: {}", Reporter(source));
                StatusCode::BAD_GATEWAY.into_response()
            }
            FetchConfigError::QueueNotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

/// Extract data about the queue config
///
/// This will contact the config service to get the queue config, and add it
/// to the reques as an extension
#[debug_middleware(state = crate::App)]
pub async fn extract_queue_config(
    State(db): State<PgPool>,
    State(services): State<Arc<Services>>,
    client: ServicesClient,
    Path(QueuePathParams { id: queue_id }): Path<QueuePathParams>,
    mut request: Request,
    next: Next,
) -> Result<Response, FetchConfigError> {
    // Get the queue config id
    let config_id: Uuid = sqlx::query_scalar("SELECT config_id FROM queue WHERE id = $1")
        .bind(queue_id)
        .fetch_optional(&db)
        .await
        .context(SQLSnafu)?
        .context(QueueNotFoundSnafu)?;

    // Get the config from the config service
    let config: QueueConfig = client
        .get(services.configs_url.join(&config_id.to_string()).unwrap())
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    request.extensions_mut().insert(Arc::new(config));

    Ok(next.run(request).await)
}
