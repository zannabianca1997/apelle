use apelle_common::{
    common_errors::{SQLError, SQLSnafu},
    paginated::{PageInfo, Paginated, PaginationParams},
};
use apelle_songs_dtos::source::{Source, SourceRegister};
use axum::{
    Json, debug_handler,
    extract::{Query, State},
    response::NoContent,
};
use futures::FutureExt;
use snafu::ResultExt;
use sqlx::{PgPool, Row, postgres::PgRow};

#[debug_handler(state=crate::App)]
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
        tracing::debug!(urn, name, "Source already registered");
    } else {
        tracing::debug!(urn, name, "Source registered");
    }

    Ok(NoContent)
}

#[debug_handler(state=crate::App)]
pub async fn list(
    State(db): State<PgPool>,
    Query(PaginationParams { page, page_size }): Query<PaginationParams>,
) -> Result<Json<Paginated<Source>>, SQLError> {
    let page = page.unwrap_or(0);
    let page_size = page_size.unwrap_or(10);
    let offset = page.saturating_mul(page_size);

    // Using LIMIT OFFSET, as there are few sources ( probably less than a single page)
    let items = sqlx::query(
        "SELECT urn, name, created, last_heard FROM source ORDER BY urn DESC LIMIT $1 OFFSET $2",
    )
    .bind(page_size as i64)
    .bind(offset as i64)
    .map(|row: PgRow| Source {
        urn: row.get(0),
        name: row.get(1),
        created: row.get(2),
        last_heard: row.get(3),
    })
    .fetch_all(&db)
    .map(|r| r.context(SQLSnafu));

    let total = sqlx::query_scalar("SELECT COUNT(*) FROM source")
        .fetch_one(&db)
        .map(|r| r.context(SQLSnafu));

    let (items, total): (_, i64) = tokio::try_join!(items, total)?;
    let total = total as u32;

    Ok(Json(Paginated {
        page_info: PageInfo::regular(page, Some(total), items.len() as u32, page_size),
        items,
    }))
}
