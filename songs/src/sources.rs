use apelle_common::common_errors::{SQLError, SQLSnafu};
use apelle_songs_dtos::source::SourceRegister;
use axum::{
    Json, debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, NoContent},
};
use snafu::{ResultExt, Snafu};
use sqlx::PgPool;

#[debug_handler]
pub async fn register(
    State(db): State<PgPool>,
    Json(SourceRegister { urn, name }): Json<SourceRegister>,
) -> Result<NoContent, SQLError> {
    tracing::info!(urn, name, "Registering source");

    let rows = sqlx::query(
        "
                INSERT INTO source (urn, name)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING
            ",
    )
    .bind(&urn)
    .bind(&name)
    .execute(&db)
    .await
    .context(SQLSnafu)?
    .rows_affected();

    if rows == 0 {
        tracing::info!(urn, name, "Source already registered");
    } else {
        tracing::info!(urn, name, "Source registered");
    }

    Ok(NoContent)
}

#[derive(Debug, Snafu)]
pub enum PingError {
    #[snafu(transparent)]
    SQLError {
        source: SQLError,
    },
    UnknownSource {
        urn: String,
    },
}

impl IntoResponse for PingError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::SQLError { source } => source.into_response(),
            Self::UnknownSource { .. } => StatusCode::NOT_FOUND.into_response(),
        }
    }
}
