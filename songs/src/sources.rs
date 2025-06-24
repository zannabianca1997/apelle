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

/// Register a new source
///
/// This will add a new source to the ones available for solving. If the source
/// already exists, nothing will be done, and no error will be returned.
#[debug_handler(state=crate::App)]
#[utoipa::path(
    post,
    path = "/sources",
    responses(
        (status = StatusCode::NO_CONTENT, description = "Source registered"),
        SQLError
    )
)]
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

/// List available sources
///
/// Get a paginated list of sources. The sources are alphabetically ordered.
#[debug_handler(state=crate::App)]
#[utoipa::path(
    get,
    path = "/sources",
    responses(
        (status = StatusCode::OK, description = "List of sources", body = Paginated<Source>),
        SQLError
    ),
    params(PaginationParams)
)]
pub async fn list(
    State(db): State<PgPool>,
    Query(PaginationParams { page, page_size }): Query<PaginationParams>,
) -> Result<Json<Paginated<Source>>, SQLError> {
    let page = page.unwrap_or(0);

    // Using LIMIT OFFSET, as there are few sources (probably less than a single
    // page) and they have a easy order
    let items = sqlx::query(
        "SELECT urn, name, created, last_heard FROM source ORDER BY urn DESC LIMIT $1 OFFSET $2",
    )
    .bind(page_size as i64)
    .bind(page as i64)
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
    let size = items.len() as u32;
    let end = page.saturating_add(size);

    debug_assert!(end <= total);

    Ok(Json(Paginated {
        page_info: PageInfo {
            size,
            total: Some(total),
            first: Some(0),
            prev: (page > 0).then(|| page.saturating_sub(page_size)),
            page,
            next: (end < total).then_some(end),
            last: Some(total.saturating_sub(page_size)),
        },
        items,
    }))
}
