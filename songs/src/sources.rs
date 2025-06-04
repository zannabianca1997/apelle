use apelle_common::common_errors::{SQLError, SQLSnafu};
use apelle_songs_dtos::source::SourceRegister;
use axum::{Json, debug_handler, extract::State, response::NoContent};
use snafu::ResultExt;
use sqlx::PgPool;

#[debug_handler]
pub async fn register(
    State(db): State<PgPool>,
    Json(SourceRegister { urn, name }): Json<SourceRegister>,
) -> Result<NoContent, SQLError> {
    tracing::info!(urn, name, "Registering source");

    let rows = sqlx::query("INSERT INTO source (urn, name) VALUES ($1, $2) ON CONFLICT DO NOTHING")
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
