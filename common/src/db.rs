use axum::response::IntoResponse;
use reqwest::StatusCode;
use snafu::Snafu;
use url::Url;
use utoipa::IntoResponses;

use crate::Reporter;

pub type SqlTx = axum_sqlx_tx::Tx<sqlx::Postgres, SqlError>;
pub type SqlState = axum_sqlx_tx::State<sqlx::Postgres>;
pub type SqlTxLayer = axum_sqlx_tx::Layer<sqlx::Postgres, SqlError>;

pub mod migrations;

#[derive(Debug, Snafu)]
pub enum SqlError {
    #[snafu(transparent)]
    Extractor { source: axum_sqlx_tx::Error },
    #[snafu(transparent)]
    Sql { source: sqlx::Error },
}

impl IntoResponse for SqlError {
    fn into_response(self) -> axum::response::Response {
        match self {
            SqlError::Extractor { source } => {
                tracing::error!("Error in creating transaction: {}", Reporter(source))
            }
            SqlError::Sql { source } => {
                tracing::error!("SQL error: {}", Reporter(source))
            }
        }

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl IntoResponses for SqlError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [(
            StatusCode::INTERNAL_SERVER_ERROR.as_str().to_string(),
            utoipa::openapi::response::Response::new("Internal server error").into(),
        )]
        .into_iter()
        .collect()
    }
}

#[derive(Debug, Snafu, derive_more::From)]
pub enum InitError {
    Connect { source: sqlx::Error },
    Migration { source: migrations::Error },
}

pub async fn db_state_and_layer(
    url: Url,
    migrations: &'static migrations::Migrations,
    migrations_config: &migrations::MigrationsConfigs,
) -> Result<(SqlState, SqlTxLayer), InitError> {
    tracing::info!(%url,"Connecting to database");
    let db = sqlx::PgPool::connect(url.as_str()).await?;

    migrations
        .check(db.acquire().await?.detach(), &migrations_config)
        .await?;

    Ok(SqlTx::config(db).layer_error::<SqlError>().setup())
}
