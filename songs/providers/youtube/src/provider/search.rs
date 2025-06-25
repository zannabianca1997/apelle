use std::{collections::HashMap, sync::Arc};

use apelle_common::{
    Reporter, ServicesClient,
    common_errors::{CacheError, CacheSnafu},
    db::{SqlError, SqlTx},
    normalize_query,
    paginated::{PageInfo, Paginated, PaginationParams},
};
use apelle_songs_dtos::{provider::SearchQueryParams, public::SearchResponseItemState};
use axum::{
    Json, debug_handler,
    extract::{Query, State},
    response::IntoResponse,
};
use const_format::concatcp;
use redis::{AsyncCommands, aio::ConnectionManager};
use reqwest::StatusCode;
use serde::Serialize;
use snafu::{ResultExt, Snafu};
use sqlx::Row as _;
use uuid::Uuid;

use crate::{
    YoutubeApi,
    provider::{dtos::ResolveRequest, video_url},
};

use super::{
    GOOGLE_API_KEY_HEADER,
    dtos::{SearchItemDetails, SearchResponseItem, youtube},
};

const CACHE_NAMESPACE: &str = concatcp!(crate::CACHE_NAMESPACE, "search:");

#[derive(Debug, Snafu)]
pub enum SearchError {
    #[snafu(transparent)]
    SQLError {
        source: SqlError,
    },
    #[snafu(transparent)]
    CacheError {
        source: CacheError,
    },
    #[snafu(transparent)]
    RequestError {
        source: reqwest::Error,
    },
    #[snafu(display("Failed to deserialize json present in the cache"))]
    CacheJsonError {
        source: serde_json::Error,
    },
    TooManyUpstreams,
    PageTokenNegative,
}

impl IntoResponse for SearchError {
    fn into_response(self) -> axum::response::Response {
        match self {
            SearchError::SQLError { source } => source.into_response(),
            SearchError::CacheError { source } => source.into_response(),
            SearchError::CacheJsonError { source } => {
                tracing::error!(
                    "Failed to deserialize json present in the cache: {}",
                    Reporter(source)
                );
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            SearchError::RequestError { source } => {
                tracing::error!("Request to the youtube api error: {}", Reporter(source));
                StatusCode::BAD_GATEWAY.into_response()
            }
            SearchError::TooManyUpstreams => {
                tracing::warn!("Request failed for too many upstream requests");
                StatusCode::BAD_REQUEST.into_response()
            }
            SearchError::PageTokenNegative => {
                tracing::warn!("Bad page token: pointing to before the first page");
                StatusCode::BAD_REQUEST.into_response()
            }
        }
    }
}

#[derive(Debug, Serialize, Clone, Copy)]
struct YoutubeQueryParams<'a> {
    part: &'a str,
    r#type: &'a str,
    #[serde(rename = "safeSearch")]
    safe_search: &'a str,
    #[serde(rename = "videoEmbeddable")]
    video_embeddable: &'a str,
    #[serde(rename = "maxResults")]
    max_results: u32,
    #[serde(rename = "q")]
    query: &'a str,
    #[serde(rename = "pageToken")]
    page_token: Option<&'a str>,
}

mod cursor;
use cursor::Cursor;

#[debug_handler(state=crate::App)]
pub async fn search(
    State(mut cache): State<ConnectionManager>,
    mut tx: SqlTx,
    client: ServicesClient,
    State(youtube_api): State<Arc<YoutubeApi>>,
    Query(SearchQueryParams { query }): Query<SearchQueryParams>,
    Query(PaginationParams { page, page_size }): Query<PaginationParams<Cursor>>,
) -> Result<Json<Paginated<SearchResponseItem, Cursor>>, SearchError> {
    // Normalize the query
    let query = normalize_query(&query);

    // Cache namespace for this query
    let mut items = Vec::with_capacity(page_size as usize);

    // Semaphore to limit the number of requests to the upstream
    let mut allowed_upstreams = youtube_api.max_upstream_requests;

    // Unpacking the cursor
    let mut page = page.unwrap_or_else(Cursor::new);

    // Fetching the first page
    let mut fetched_page = fetch_page(
        &mut cache,
        &client,
        &youtube_api,
        &query,
        page.page.as_deref(),
        &mut allowed_upstreams,
    )
    .await?;

    let total = fetched_page.page_info.total_results;

    // Paging backwards until we reach the wanted page, if a negative offset was provided
    while page.offset < 0 {
        if fetched_page.prev_page_token.is_none() {
            // A page was requested that starts before the first page
            tracing::warn!(
                page.page,
                page.offset,
                "Fetched a page that starts before the first page"
            );
            return Err(SearchError::PageTokenNegative);
        }
        page.page = fetched_page.prev_page_token.take();

        fetched_page = fetch_page(
            &mut cache,
            &client,
            &youtube_api,
            &query,
            page.page.as_deref(),
            &mut allowed_upstreams,
        )
        .await?;
        page.offset += fetched_page.items.len() as i32;
    }

    let prev_page = if page.offset > 0 || fetched_page.prev_page_token.is_some() {
        Some(Cursor {
            page: page.page.clone(),
            offset: page.offset - page_size as i32,
        })
    } else {
        None
    };

    let page = page;

    let mut current_page_token = page.page.clone();

    let mut next_page = None;
    let mut skipping = page.offset;

    'fill: loop {
        let next_page_token = fetched_page.next_page_token.take();

        for (i, item) in fetched_page.into_iter().enumerate() {
            if skipping > 0 {
                skipping -= 1;
                continue;
            }

            items.push(item);

            if items.len() >= page_size as usize {
                // Stopping

                next_page = Some(Cursor {
                    page: current_page_token,
                    offset: (i + 1) as i32,
                });

                break 'fill;
            }
        }

        let Some(next_page_token) = next_page_token else {
            break;
        };

        fetched_page = fetch_page(
            &mut cache,
            &client,
            &youtube_api,
            &query,
            Some(&next_page_token),
            &mut allowed_upstreams,
        )
        .await?;
        current_page_token = Some(next_page_token);
    }

    // Detecting known songs and finding their ids

    let video_ids: Vec<_> = items
        .iter()
        .map(|i| {
            if let youtube::SearchResultId::Video { video_id } = &i.id {
                Some(video_id.as_str())
            } else {
                None
            }
        })
        .collect();

    let known_ids: HashMap<String, Uuid> =
        sqlx::query("SELECT video_id, id FROM youtube_song WHERE video_id = ANY($1)")
            .bind(&video_ids)
            .map(|row| (row.get(0), row.get(1)))
            .fetch_all(&mut tx)
            .await
            .map_err(SqlError::from)?
            .into_iter()
            .collect();

    // Converting the items to our internal representation

    let items: Vec<_> = items
        .into_iter()
        .filter_map(
            |youtube::SearchResult {
                 id,
                 snippet: youtube::Snippet { title, thumbnails },
             }| {
                let youtube::SearchResultId::Video { video_id } = id else {
                    // There is a bug in the youtube api where it returns a
                    // playlist or a channel if the query perfectly matches
                    // the title. Clearing out them from the list.
                    return None;
                };
                Some(SearchResponseItem {
                    details: SearchItemDetails {
                        title,
                        url: video_url(&youtube_api.public_url, &video_id),
                        thumbnails: thumbnails.into_values().map(|t| t.into()).collect(),
                    },
                    state: if let Some(&id) = known_ids.get(&video_id) {
                        SearchResponseItemState::Known { id }
                    } else {
                        SearchResponseItemState::New {
                            data: ResolveRequest { video_id },
                        }
                    },
                })
            },
        )
        .collect();

    Ok(Json(Paginated {
        page_info: PageInfo {
            size: items.len() as u32,
            total: Some(total),
            first: Some(Cursor::new()),
            prev: prev_page,
            next: next_page,
            last: None,
            page,
        },
        items,
    }))
}

/// Get a page either from the cache or from youtube
async fn fetch_page(
    cache: &mut ConnectionManager,
    client: &ServicesClient,
    youtube_api: &Arc<YoutubeApi>,
    query: &str,
    page: Option<&str>,
    allowed_upstreams: &mut u32,
) -> Result<youtube::Paginated<youtube::SearchResult>, SearchError> {
    let request = client
        .get(youtube_api.api_search_url.clone())
        .query(&YoutubeQueryParams {
            part: "snippet",
            r#type: "video",
            safe_search: "none",
            video_embeddable: "true",
            max_results: youtube_api.page_size,
            query,
            page_token: page,
        })
        .header(GOOGLE_API_KEY_HEADER, &youtube_api.api_key)
        .build()?;

    let cache_key = CACHE_NAMESPACE.to_owned() + request.url().query().unwrap_or_default();

    // Try to get the page from the cache
    let cached = cache
        .get::<_, Option<String>>(&cache_key)
        .await
        .context(CacheSnafu)?;

    if let Some(cached) = cached {
        return serde_json::from_str(&cached).context(CacheJsonSnafu);
    }

    // Fetch the page from youtube

    // Get a permit to use the upstream, and immediately forget it to limit the
    // total number of requests to the upstream
    if *allowed_upstreams == 0 {
        return Err(SearchError::TooManyUpstreams);
    }
    *allowed_upstreams -= 1;

    let page = client.client().execute(request).await?.json().await?;

    // Store the page in the cache

    // TODO: This can be done in a separate taks, so we don't block the request
    // for a cache roundtrip
    let _: () = cache
        .set_ex(
            &cache_key,
            serde_json::to_string(&page).context(CacheJsonSnafu)?,
            youtube_api.expiration.num_seconds() as _,
        )
        .await
        .context(CacheSnafu)?;

    Ok(page)
}
