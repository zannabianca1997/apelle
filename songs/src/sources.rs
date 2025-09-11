use apelle_common::{
    db::{SqlError, SqlTx},
    paginated::{PageInfo, Paginated, PaginationParams},
};
use apelle_songs_dtos::source::{Source, SourceRegister};
use axum::{Json, debug_handler, extract::Query, response::NoContent};

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
        SqlError
    )
)]
pub async fn register(
    mut tx: SqlTx,
    Json(SourceRegister { urn, name }): Json<SourceRegister>,
) -> Result<NoContent, SqlError> {
    tracing::info!(urn, name, "Registering source");

    let rows = sqlx::query!(
        "INSERT INTO source (urn, name) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        &urn,
        &name
    )
    .execute(&mut tx)
    .await?
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
        SqlError
    ),
    params(PaginationParams)
)]
pub async fn list(
    mut tx: SqlTx,
    Query(PaginationParams { page, page_size }): Query<PaginationParams>,
) -> Result<Json<Paginated<Source>>, SqlError> {
    let page = page.unwrap_or(0);

    // Using LIMIT OFFSET, as there are few sources (probably less than a single
    // page) and they have a easy order
    let items = sqlx::query!(
        "SELECT urn, name, created, last_heard FROM source ORDER BY urn DESC LIMIT $1 OFFSET $2",
        page_size as i64,
        page as i64
    )
    .map(|row| Source {
        urn: row.urn,
        name: row.name,
        created: row.created.into(),
        last_heard: row.last_heard.map(Into::into),
    })
    .fetch_all(&mut tx)
    .await?;

    let total: i64 = sqlx::query_scalar!(r#"SELECT COUNT(*) AS "total!" FROM source"#)
        .fetch_one(&mut tx)
        .await?;

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
