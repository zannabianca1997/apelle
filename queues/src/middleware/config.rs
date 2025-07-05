use std::sync::Arc;

use apelle_common::{
    Reporter, ServicesClient,
    db::{SqlError, SqlTx},
};
use apelle_configs_dtos::QueueConfig;
use axum::{
    extract::{Path, Request, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use snafu::{OptionExt, Snafu};
use uuid::Uuid;

use crate::{QueuePathParams, Services};

#[derive(Debug, Snafu)]
pub enum FetchConfigError {
    #[snafu(transparent)]
    SqlError {
        source: SqlError,
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
pub async fn extract_queue_config(
    mut tx: SqlTx,
    State(services): State<Arc<Services>>,
    client: ServicesClient,
    Path(QueuePathParams { id: queue_id }): Path<QueuePathParams>,
    mut request: Request,
) -> Result<Request, FetchConfigError> {
    // Get the queue config id
    let config_id: Uuid = sqlx::query_scalar("SELECT config_id FROM queue WHERE id = $1")
        .bind(queue_id)
        .fetch_optional(&mut tx)
        .await
        .map_err(SqlError::from)?
        .context(QueueNotFoundSnafu)?;

    // Get the config from the config service
    let config: QueueConfig = client
        .get(
            services
                .configs_url
                .join(&format!("queues/{config_id}"))
                .unwrap(),
        )
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    request.extensions_mut().insert(Arc::new(config));

    Ok(request)
}
