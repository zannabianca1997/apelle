use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};

use apelle_common::{
    Reporter, TracingClient,
    common_errors::{CacheError, CacheSnafu, SQLError, SQLSnafu},
    paginated::{PageInfo, Paginated, PaginationParams},
};
use apelle_songs_dtos::provider::{SearchQueryParams, SearchResponseItemState};
use axum::{
    Json, debug_handler,
    extract::{Query, State},
    response::IntoResponse,
};
use const_format::concatcp;
use futures::{FutureExt as _, TryFutureExt as _};
use itertools::Itertools;
use redis::{AsyncCommands, aio::ConnectionManager};
use reqwest::{Response, StatusCode};
use serde::Serialize;
use snafu::{ResultExt, Snafu};
use sqlx::{PgPool, Row as _};
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

const FIRST_PAGE_NAMESPACE: &str = "first-page:";
const PAGES_NAMESPACE: &str = "pages:";
const TOTAL_KEY: &str = "total";

const ITEMS_KEY: &str = "items";
const NEXT_PAGE_KEY: &str = "next-page";

#[derive(Debug, Snafu)]
pub enum SearchError {
    #[snafu(transparent)]
    SQLError {
        source: SQLError,
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

    PageTooFar,

    #[snafu(display("The cache server did not return the correct number of items"))]
    CacheLRangeLengthMismatch {
        cache_key: String,
        requested: usize,
        returned: usize,
    },
}

impl IntoResponse for SearchError {
    fn into_response(self) -> axum::response::Response {
        match self {
            SearchError::SQLError { source } => source.into_response(),
            SearchError::CacheError { source } => source.into_response(),
            SearchError::PageTooFar => StatusCode::BAD_REQUEST.into_response(),
            SearchError::CacheJsonError { source } => {
                tracing::error!(
                    "Failed to deserialize json present in the cache: {}",
                    Reporter(source)
                );
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            SearchError::CacheLRangeLengthMismatch {
                cache_key,
                requested,
                returned,
            } => {
                tracing::error!(
                    cache_key,
                    requested,
                    returned,
                    "The cache server did not return the correct number of items"
                );
                StatusCode::BAD_GATEWAY.into_response()
            }
            SearchError::RequestError { source } => {
                tracing::error!("Request to the youtube api error: {}", Reporter(source));
                StatusCode::BAD_GATEWAY.into_response()
            }
        }
    }
}

#[derive(Debug, Serialize)]
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
    page_token: Option<String>,
}

struct ItemFetcher<'a> {
    query: &'a str,

    api: &'a YoutubeApi,
    cache: ConnectionManager,
    client: &'a TracingClient,

    pages_namespace: String,

    /// The number of calls remaining before the quota for this query is exhausted
    calls_remaining: AtomicU32,
}

impl ItemFetcher<'_> {
    /// Fetch a range of items from the cache or youtube
    async fn fetch(
        &self,
        cache_key: String,
        offset: u32,
        buffer: &mut [Option<SearchResponseItem>],
        current_page_token: Option<String>,
        total: &AtomicU32,
    ) -> Result<usize, SearchError> {
        // Fetch the current page length
        let (cached_page_len, next_page_token): (u32, Option<String>) = redis::pipe()
            .llen(cache_key.clone() + ITEMS_KEY)
            .get(cache_key.clone() + NEXT_PAGE_KEY)
            .query_async(&mut self.cache.clone())
            .await
            .context(CacheSnafu)?;

        tracing::debug!(
            cached_page_len,
            next_page_token,
            "Fetched page info from cache"
        );

        if cached_page_len == 0 {
            // Page never fetched
            if next_page_token.is_some() {
                tracing::warn!(
                    %cache_key,
                    "Page never fetched, but next page token is present"
                )
            }

            // Fetch from youtube
            return self
                .fetch_from_yt(cache_key.clone(), offset, buffer, current_page_token, total)
                .await;
        }

        // If part of the page is after this, fetch the next one
        let (inside_cached_page, outside_cached_page) =
            break_buffer(cached_page_len, offset, buffer);

        let fetch_outside_cached_page = async {
            if let (Some((offset, buffer)), Some(next_page_token)) =
                (outside_cached_page, next_page_token)
            {
                let cache_key = self.pages_namespace.clone() + &next_page_token;

                // Recursively fetch the next cached page
                Box::pin(self.fetch(cache_key, offset, buffer, Some(next_page_token), total)).await
            } else {
                // Either the page is inside the cached page, or there is no next page so we ended the research
                Ok(0)
            }
        };

        let fetch_inside_cached_page = async {
            if let Some((offset, buffer)) = inside_cached_page {
                self.fetch_from_cache(cache_key, offset, buffer).await
            } else {
                Ok(0)
            }
        };

        let (fetched_from_cache, fetched_from_yt) =
            tokio::try_join!(fetch_inside_cached_page, fetch_outside_cached_page)?;

        Ok(fetched_from_cache + fetched_from_yt)
    }

    async fn fetch_from_cache(
        &self,
        cache_key: String,
        offset: u32,
        buffer: &mut [Option<SearchResponseItem>],
    ) -> Result<usize, SearchError> {
        tracing::debug!(cache_key, item_count = buffer.len(), "Fetching from cache");

        let cached_items = self
            .cache
            .clone()
            .lrange::<_, Vec<String>>(
                cache_key.clone() + ITEMS_KEY,
                offset as _,
                offset as isize + buffer.len() as isize - 1,
            )
            .await
            .context(CacheSnafu)?;

        if buffer.len() != cached_items.len() {
            return Err(SearchError::CacheLRangeLengthMismatch {
                cache_key,
                requested: buffer.len(),
                returned: cached_items.len(),
            });
        }

        for (buf, item) in buffer.iter_mut().zip(cached_items) {
            *buf = Some(serde_json::from_str(&item).context(CacheJsonSnafu)?);
        }

        Ok(buffer.len())
    }

    async fn fetch_from_yt(
        &self,
        cache_key: String,
        offset: u32,
        buffer: &mut [Option<SearchResponseItem>],
        page_token: Option<String>,
        total: &AtomicU32,
    ) -> Result<usize, SearchError> {
        if self
            .calls_remaining
            .fetch_sub(1, Ordering::Relaxed)
            .saturating_mul(self.api.page_size)
            < offset.saturating_add(buffer.len() as _)
        {
            // The page is too far, even if youtube always answers with the max numbers of results we would not reach it.
            return Err(SearchError::PageTooFar);
        };

        tracing::debug!(
            cache_key,
            item_count = buffer.len(),
            page_token,
            "Fetching from youtube"
        );

        let youtube::Paginated {
            items,
            next_page_token,
            page_info: youtube::PageInfo { total_results },
        } = self
            .client
            .get(self.api.api_search_url.clone())
            .query(&YoutubeQueryParams {
                part: "snippet",
                r#type: "video",
                safe_search: "none",
                video_embeddable: "true",
                max_results: self.api.page_size,
                query: self.query,
                page_token,
            })
            .header(GOOGLE_API_KEY_HEADER, &self.api.api_key)
            .send()
            .map(|r| r.and_then(Response::error_for_status))
            .and_then(Response::json)
            .await?;

        // Updating total
        total.store(total_results, Ordering::Relaxed);

        let mut fetched = 0;

        // Mapping the search results to our internal representation
        let search_results: Vec<_> = items
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
                            url: video_url(&self.api.public_url, &video_id),
                            thumbnails: thumbnails.into_iter().map(|(_, t)| t.into()).collect(),
                        },
                        state: SearchResponseItemState::New {
                            resolve: ResolveRequest { video_id },
                        },
                    })
                },
            )
            // Storing them
            .enumerate()
            .map(|(i, item)| {
                let serialized_for_cache =
                    serde_json::to_string(&item).expect("The serialization should be infallible");

                // Filling the buffer as we pass the window
                if offset as usize <= i && i < offset as usize + buffer.len() {
                    buffer[i - offset as usize] = Some(item);
                    fetched += 1;
                }

                serialized_for_cache
            })
            .collect();

        // Caching the results of the query
        let cache_update = async {
            let items_key = cache_key.clone() + ITEMS_KEY;
            let expiration = self.api.expiration.num_seconds() as u64;
            let mut pipe = redis::pipe();
            pipe.atomic()
                .lpush(&items_key, &search_results)
                .expire(items_key, expiration as i64);
            if let Some(next_page_token) = next_page_token.as_ref() {
                pipe.set_ex(
                    cache_key.clone() + NEXT_PAGE_KEY,
                    next_page_token,
                    expiration,
                );
            }
            pipe.exec_async(&mut self.cache.clone())
                .await
                .context(CacheSnafu)?;
            Ok(())
        };

        // If there are more element to fetch, call again the API
        let fill_rest_of_buffer = async {
            let (_, outside_fetched_page) = break_buffer(search_results.len() as _, offset, buffer);
            if let (Some((offset, buffer)), Some(next_page_token)) =
                (outside_fetched_page, next_page_token.as_ref())
            {
                tracing::debug!(%next_page_token, "Fetching next page");

                let cache_key = self.pages_namespace.clone() + &next_page_token;

                Box::pin(self.fetch_from_yt(
                    cache_key,
                    offset,
                    buffer,
                    Some(next_page_token.clone()),
                    total,
                ))
                .await
            } else {
                Ok(0)
            }
        };

        let ((), n) = tokio::try_join!(cache_update, fill_rest_of_buffer)?;

        Ok(fetched + n)
    }
}

/// Splits the buffer into two parts, one that is below the breakpoint and one that is above it
fn break_buffer(
    breakpoint: u32,
    offset: u32,
    buffer: &mut [Option<SearchResponseItem>],
) -> (
    Option<(u32, &mut [Option<SearchResponseItem>])>,
    Option<(u32, &mut [Option<SearchResponseItem>])>,
) {
    let (inside_cached_page, outside_cached_page) =
        if offset.saturating_add(buffer.len() as _) <= breakpoint {
            // The entire page is inside the cached page
            (Some((offset, buffer)), None)
        } else if offset >= breakpoint {
            // The entire page is outside the cached page
            (None, Some((offset - breakpoint, buffer)))
        } else {
            // Part of the page is inside the cached page
            let (b1, b2) = buffer.split_at_mut((breakpoint - offset) as usize);
            (Some((offset, b1)), Some((0, b2)))
        };
    (inside_cached_page, outside_cached_page)
}

#[debug_handler(state=crate::App)]
pub async fn search(
    State(mut cache): State<ConnectionManager>,
    State(db): State<PgPool>,
    client: TracingClient,
    State(youtube_api): State<Arc<YoutubeApi>>,
    Query(SearchQueryParams { query }): Query<SearchQueryParams>,
    Query(PaginationParams { page, page_size }): Query<PaginationParams>,
) -> Result<Json<Paginated<SearchResponseItem>>, SearchError> {
    // Normalize the query
    let query = normalize_query(&query);

    // Calculate the offsets
    let page = page.unwrap_or(0);
    let page_start = page.saturating_mul(page_size);
    let page_end = page_start.saturating_add(page_size);

    // Cache namespace for this query
    let cache_namespace = CACHE_NAMESPACE.to_owned() + &query.replace(' ', "-") + ":";

    let mut items = vec![None; page_size as usize];
    let total = AtomicU32::new(u32::MAX);

    let fetched = ItemFetcher {
        query: &query,
        api: &*youtube_api,
        cache: cache.clone(),
        client: &client,
        pages_namespace: cache_namespace.clone() + PAGES_NAMESPACE,
        calls_remaining: AtomicU32::new(youtube_api.max_upstream_requests),
    }
    .fetch(
        cache_namespace.clone() + FIRST_PAGE_NAMESPACE,
        page_start,
        &mut items,
        None,
        &total,
    )
    .await?;

    let fetch_total = async {
        let total_key = cache_namespace + TOTAL_KEY;
        let total = total.into_inner();
        if total == u32::MAX {
            // No page has been fetched, must get from the cache We are sure
            // that the key is present as the only way the search concluded
            // without asking the yt api is that the cache is filled
            cache.get(total_key).await.context(CacheSnafu)
        } else {
            // Caching the total for the next time. This will extend the time to
            // live of the cache, but the data will be invalidated anyway. The
            // key will disappear on his own or will be overwritten
            cache
                .set_ex::<_, _, ()>(total_key, total, youtube_api.expiration.num_seconds() as _)
                .await
                .context(CacheSnafu)?;
            Ok(total)
        }
    }
    .map_err(|source| SearchError::CacheError { source });

    let detect_known = async {
        let items: Vec<_> = items
            .drain(..fetched)
            .map(|i| i.expect("All the items should be filled by the fetcher"))
            .collect();

        // Detecting known songs
        let video_ids: Vec<_> = items
            .iter()
            .map(|i| {
                let SearchResponseItemState::New {
                    resolve: ResolveRequest { video_id, .. },
                } = &i.state
                else {
                    unreachable!("Detection still did not run");
                };
                video_id.as_str()
            })
            .collect();

        let known_ids: HashMap<String, Uuid> =
            sqlx::query("SELECT video_id, id FROM youtube_song WHERE video_id = ANY($1)")
                .bind(&video_ids)
                .map(|row| (row.get(0), row.get(1)))
                .fetch_all(&db)
                .await
                .context(SQLSnafu)?
                .into_iter()
                .collect();

        let mut items = items;
        for i in items.iter_mut() {
            let SearchResponseItemState::New {
                resolve: ResolveRequest { video_id, .. },
            } = &i.state
            else {
                unreachable!("Detection still did not run");
            };
            if let Some(id) = known_ids.get(video_id).copied() {
                i.state = SearchResponseItemState::Known { id };
            }
        }

        Ok(items)
    };

    let (total, items) = tokio::try_join!(fetch_total, detect_known)?;

    Ok(Json(Paginated {
        page_info: PageInfo::regular(
            page,
            Some(total),
            items.len() as _,
            page_size,
            page_end < total,
        ),
        items,
    }))
}

/// Normalize a query to help with caching
///
/// This will:
/// - Remove non-alphanumeric characters (by unicode definition, so kanji & co. are not removed)
/// - Replace multiple spaces with a single space
/// - Remove leading and trailing spaces
/// - Convert to lowercase
fn normalize_query(query: &str) -> String {
    query
        // Remove non-alphanumeric characters
        .replace(|ch: char| !ch.is_alphanumeric(), " ")
        // Replace multiple spaces with a single space
        .split_whitespace()
        .join(" ")
        // Remove leading and trailing spaces
        .trim()
        // Convert to lowercase
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    mod normalize_query {
        use super::super::*;

        #[test]
        fn should_make_query_lowercase() {
            assert_eq!(normalize_query("Hello World"), "hello world");
        }

        #[test]
        fn should_remove_non_alphanumeric_characters() {
            assert_eq!(normalize_query("hello\nworld ! 42"), "hello world 42");
        }

        #[test]
        fn should_preserve_unicode() {
            assert_eq!(normalize_query("漢字 漢字"), "漢字 漢字");
            assert_eq!(normalize_query("۱ ۲ ۳ ۴ ۵ ۶ ۷ ۸ ۹"), "۱ ۲ ۳ ۴ ۵ ۶ ۷ ۸ ۹");
        }

        #[test]
        fn should_replace_multiple_spaces_with_a_single_space() {
            assert_eq!(normalize_query("  hello    world  "), "hello world");
        }
    }
}
