pub mod auth;
pub mod cache_pubsub;
mod cli;
pub mod common_errors;
mod error_reporter;
pub mod id_or_rep;
pub mod iso8601;
mod logging;
mod main_wrapper;
pub mod paginated;
mod search;
mod serve;
mod services_client;
pub mod db {
    use axum::response::IntoResponse;
    use reqwest::StatusCode;
    use snafu::Snafu;
    use url::Url;
    use utoipa::IntoResponses;

    use crate::Reporter;

    pub type SqlTx = axum_sqlx_tx::Tx<sqlx::Postgres, SqlError>;
    pub type SqlState = axum_sqlx_tx::State<sqlx::Postgres>;
    pub type SqlTxLayer = axum_sqlx_tx::Layer<sqlx::Postgres, SqlError>;

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

    pub async fn db_state_and_layer(url: Url) -> Result<(SqlState, SqlTxLayer), sqlx::Error> {
        tracing::info!(%url,"Connecting to database");
        let db = sqlx::PgPool::connect(url.as_str()).await?;

        Ok(SqlTx::config(db).layer_error::<SqlError>().setup())
    }
}

pub use auth::AuthHeaders;
pub use cli::ProvideDefaults;
pub use error_reporter::Reporter;
pub use figment::{
    Figment, Provider, map as figment_map, providers::Serialized, value::magic::RelativePathBuf,
};
pub use main_wrapper::Error;
pub use main_wrapper::{
    PUBLIC_TAG, SERVICE_TAG, iter_operations, iter_operations_mut, service_main,
};
pub use search::normalize_query;
pub use services_client::ServicesClient;
