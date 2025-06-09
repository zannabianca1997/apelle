use std::{
    collections::{BTreeSet, HashSet},
    hash::{Hash, Hasher},
    iter::repeat,
    string::FromUtf8Error,
    sync::atomic::{AtomicU32, Ordering},
};

use apelle_common::{
    Reporter, TracingClient,
    common_errors::{CacheError, CacheSnafu, SQLError},
    normalize_query,
    paginated::{PageInfo, Paginated, PaginationParams},
};
use axum::{
    Json, debug_handler,
    extract::{Query, State},
    response::IntoResponse,
};
use base64::{Engine, alphabet::Alphabet, engine::GeneralPurpose};
use chrono::Duration;
use const_format::concatcp;
use futures::{FutureExt as _, TryFutureExt as _, TryStreamExt};
use itertools::Itertools;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use redis::{AsyncCommands, aio::ConnectionManager};
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use snafu::{ResultExt, Snafu};
use sqlx::PgPool;
use url::Url;

use apelle_songs_dtos::public::{SearchQueryParams, SearchResponseItem, UnknownSources};

use crate::{
    ProvidersConfig,
    providers::{provider_for_urn, search_endpoint},
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
    RequestError {
        upstream: Url,
        source: reqwest::Error,
    },
    #[snafu(display("Failed to deserialize json present in the cache"))]
    CacheJsonError {
        source: serde_json::Error,
    },
    BadCachedPageToken {
        source: FromUtf8Error,
    },

    PageTooFar,

    #[snafu(display("The cache server did not return the correct number of items"))]
    CacheLRangeLengthMismatch {
        cache_key: String,
        requested: usize,
        returned: usize,
    },
    UnknownSources {
        missing: Vec<String>,
    },
    BadPage,
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
            SearchError::RequestError { source, upstream } => {
                tracing::error!(
                    %upstream,
                    "Request to the upstream provider failed: {}",
                    Reporter(source)
                );
                StatusCode::BAD_GATEWAY.into_response()
            }
            SearchError::UnknownSources { missing } => {
                (StatusCode::NOT_FOUND, Json(UnknownSources(missing))).into_response()
            }
            SearchError::BadPage => (
                StatusCode::BAD_REQUEST,
                concat!(
                    "Bad page token: either the search is too old, ",
                    "or the token does not come from a previous search"
                ),
            )
                .into_response(),
            SearchError::BadCachedPageToken { source } => {
                tracing::error!(
                    "Failed to parse urlencoded provider page token: {}",
                    Reporter(source)
                );
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

struct ItemFetcher<'a> {
    query: &'a str,

    providers: &'a BTreeSet<ProviderData>,
    cache: ConnectionManager,
    client: &'a TracingClient,

    pages_namespace: String,

    cache_expiration: Duration,

    upstream_page_size: u32,
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

            // Fetch from providers
            return self
                .fetch_from_upstreams(cache_key.clone(), offset, buffer, current_page_token, total)
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

    async fn fetch_from_upstreams(
        &self,
        cache_key: String,
        offset: u32,
        buffer: &mut [Option<SearchResponseItem>],
        page_token: Option<String>,
        total: &AtomicU32,
    ) -> Result<usize, SearchError> {
        tracing::debug!(
            cache_key,
            item_count = buffer.len(),
            page_token,
            "Fetching from upstreams"
        );

        let mut upstream_items = Vec::with_capacity(self.providers.len());
        let mut next_pages_tokens: Vec<Option<Value>> = Vec::with_capacity(self.providers.len());
        let mut total_results = Vec::with_capacity(self.providers.len());

        for (ProviderData { source, provider }, next_page) in self.providers.iter().zip(
            page_token
                .as_ref()
                .map(|p| {
                    itertools::Either::Left(p.split('/').map(|s| {
                        if s.is_empty() {
                            None
                        } else {
                            Some(
                                serde_json::from_str::<Value>(
                                    &urlencoding::decode(s).context(BadCachedPageTokenSnafu)?,
                                )
                                .context(CacheJsonSnafu),
                            )
                        }
                        .transpose()
                    }))
                })
                .unwrap_or_else(|| itertools::Either::Right(repeat(None).map(Ok))),
        ) {
            let next_page = next_page?;

            if page_token.is_some() && next_page.is_none() {
                // Exhausted provider
                next_pages_tokens.push(None);
                total_results.push(None);
                continue;
            }

            let Paginated::<_, Value> {
                items,
                page_info: PageInfo { total, next, .. },
            } = self
                .client
                .get(search_endpoint(provider))
                .query(&apelle_songs_dtos::provider::SearchQueryParamsRef { query: self.query })
                .query(&PaginationParams {
                    page: next_page,
                    page_size: self.upstream_page_size,
                })
                .send()
                .map(|r| r.and_then(|r| r.error_for_status()))
                .and_then(Response::json)
                .await
                .with_context(|_| RequestSnafu {
                    upstream: provider.clone(),
                })?;

            upstream_items.push(items.into_iter().map(
                |apelle_songs_dtos::provider::SearchResponseItem { details, state }| {
                    SearchResponseItem {
                        source: source.clone(),
                        details,
                        state,
                    }
                },
            ));

            next_pages_tokens.push(next);
            total_results.push(total);
        }

        // Mixing all the results
        let mut rng = {
            // Deterministic state based on the cache key, that includes both
            // the query and the page token.
            let mut hasher = crc32fast::Hasher::new();
            cache_key.hash(&mut hasher);
            SmallRng::seed_from_u64(hasher.finish())
        };
        let mut items = Vec::with_capacity(upstream_items.iter().map(|i| i.len()).sum());
        while !upstream_items.is_empty() {
            let upstream = rng.random_range(..upstream_items.len());
            let item = upstream_items[upstream].next();
            if let Some(item) = item {
                items.push(item);
            } else {
                let _ = upstream_items.swap_remove(upstream);
            }
        }

        let next_page_token = if next_pages_tokens.iter().all(Option::is_none) {
            None
        } else {
            Some(
                next_pages_tokens
                    .iter()
                    .map(|p| {
                        p.as_ref()
                            .map(|p| {
                                urlencoding::encode(&serde_json::to_string(p).unwrap()).into_owned()
                            })
                            .unwrap_or_default()
                    })
                    .join("/"),
            )
        };

        // If all providers gave a total, or the absence of
        if total_results.iter().all(|v| v.is_some()) {
            // Updating total
            total.store(
                total_results.into_iter().map(|t| t.unwrap()).sum(),
                Ordering::Relaxed,
            );
        }

        let mut fetched = 0;

        // Mapping the search results to our internal representation
        let search_results: Vec<_> = items
            .into_iter()
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
            let expiration = self.cache_expiration.num_seconds() as u64;
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

                Box::pin(self.fetch_from_upstreams(
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

#[derive(Debug, Clone, Copy)]
pub struct PageToken {
    search_id: [u8; 4],
    page: u32,
}

impl Serialize for PageToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = [0u8; 8];
        bytes[0..4].copy_from_slice(&self.search_id);
        bytes[4..8].copy_from_slice(&self.page.to_le_bytes());
        let value = u64::from_le_bytes(bytes);
        u64::serialize(&value, serializer)
    }
}

impl<'de> Deserialize<'de> for PageToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        let bytes = value.to_le_bytes();
        Ok(PageToken {
            search_id: bytes[0..4].try_into().unwrap(),
            page: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ProviderData {
    source: String,
    provider: Url,
}

#[debug_handler(state=crate::App)]
pub async fn search(
    State(mut cache): State<ConnectionManager>,
    State(db): State<PgPool>,
    State(ProvidersConfig {
        cache_expiration,
        page_size: upstream_page_size,
        ..
    }): State<ProvidersConfig>,
    client: TracingClient,
    axum_extra::extract::Query(SearchQueryParams { query, sources }): axum_extra::extract::Query<
        SearchQueryParams,
    >,
    Query(PaginationParams { page, page_size }): Query<PaginationParams<PageToken>>,
) -> Result<Json<Paginated<SearchResponseItem, PageToken>>, SearchError> {
    // Normalize the query
    let query = normalize_query(&query);

    // Split the page token
    let (search_id, page) = match page {
        Some(PageToken { search_id, page }) => (Some(search_id), page),
        None => (None, 0),
    };

    // Fetch the providers
    let (providers, cache_namespace, search_id) = if let Some(search_id) = search_id {
        // Existing search, getting the providers from the cache

        let cache_namespace = cache_namespace(&query, search_id);

        let providers: Vec<String> = cache
            .smembers(cache_namespace.clone() + "providers")
            .await
            .context(CacheSnafu)?;

        if providers.is_empty() {
            return Err(SearchError::BadPage);
        }

        let providers = providers
            .into_iter()
            .map(|s| serde_json::from_str(&s).context(CacheJsonSnafu))
            .collect::<Result<_, _>>()?;

        (providers, cache_namespace, search_id)
    } else {
        // New search, need to find the providers

        let sources_vec: Vec<_> = sources.iter().collect();

        let mut qb = sqlx::QueryBuilder::new("SELECT urn FROM source");
        if !sources_vec.is_empty() {
            // Using only the requested sources
            qb.push(" WHERE urn = ANY(")
                .push_bind(&sources_vec)
                .push("::text[])");
        }

        // Sources we seen from the db
        let mut seen = HashSet::new();

        let providers: BTreeSet<_> = qb
            .build_query_scalar::<String>()
            .fetch(&db)
            // Keep track of the seen sources
            .inspect_ok(|s| {
                seen.insert(s.clone());
            })
            .map_err(|source| SearchError::from(SQLError { source }))
            // Find a provider for each source
            .and_then(|urn| {
                provider_for_urn(cache.clone(), urn.clone())
                    .map_ok(|provider| ProviderData {
                        source: urn,
                        provider,
                    })
                    .map_err(Into::into)
            })
            .try_collect()
            .await?;

        if !sources.is_empty() && seen != sources {
            // Some sources are unknow
            return Err(SearchError::UnknownSources {
                missing: sources.difference(&seen).cloned().collect(),
            });
        }

        let mut hasher = crc32fast::Hasher::new();
        providers.hash(&mut hasher);
        let search_id: [u8; 4] = hasher.finalize().to_le_bytes();

        let cache_namespace = cache_namespace(&query, search_id);

        // Storing them to recover if a new page is requested
        redis::pipe()
            .atomic()
            .sadd(
                cache_namespace.clone() + "providers",
                providers
                    .iter()
                    .map(|p| serde_json::to_string(p).expect("Serialization should be infallible"))
                    .collect::<Vec<_>>(),
            )
            .expire(
                cache_namespace.clone() + "providers",
                cache_expiration.num_seconds(),
            )
            .ignore()
            .exec_async(&mut cache)
            .await
            .context(CacheSnafu)?;

        (providers, cache_namespace, search_id)
    };

    // Calculate the offsets
    let page_start = page.saturating_mul(page_size);
    let page_end = page_start.saturating_add(page_size);

    let mut items = vec![None; page_size as usize];
    let total = AtomicU32::new(u32::MAX);

    let fetched = ItemFetcher {
        query: &query,
        providers: &providers,
        cache: cache.clone(),
        client: &client,
        pages_namespace: cache_namespace.clone() + PAGES_NAMESPACE,
        cache_expiration,
        upstream_page_size,
    }
    .fetch(
        cache_namespace.clone() + FIRST_PAGE_NAMESPACE,
        page_start,
        &mut items,
        None,
        &total,
    )
    .await?;
    let items: Vec<SearchResponseItem> = items.drain(..fetched).map(Option::unwrap).collect();

    let total_key = cache_namespace + TOTAL_KEY;
    let total = total.into_inner();
    let total = if total == u32::MAX {
        // No page has been fetched, must get from the cache, if present
        cache.get(total_key).await.context(CacheSnafu)?
    } else {
        // Caching the total for the next time. This will extend the time to
        // live of the cache, but the data will be invalidated anyway. The key
        // will disappear on his own or will be overwritten
        cache
            .set_ex::<_, _, ()>(total_key, total, cache_expiration.num_seconds() as _)
            .await
            .context(CacheSnafu)?;
        Some(total)
    };

    Ok(Json(Paginated {
        page_info: {
            let PageInfo {
                size,
                total,
                page,
                first,
                prev,
                next,
                last,
            } = PageInfo::regular(
                page,
                total,
                items.len() as _,
                page_size,
                total.is_some_and(|total| page_end < total),
            );
            PageInfo {
                size,
                total,
                page,
                first: first.map(|first| PageToken {
                    search_id,
                    page: first,
                }),
                prev: prev.map(|prev| PageToken {
                    search_id,
                    page: prev,
                }),
                next: next.map(|next| PageToken {
                    search_id,
                    page: next,
                }),
                last: last.map(|last| PageToken {
                    search_id,
                    page: last,
                }),
            }
        },
        items,
    }))
}

fn cache_namespace(query: &str, search_id: [u8; 4]) -> String {
    const CACHE_B64_ENGINE: GeneralPurpose = GeneralPurpose::new(
        &{
            let Ok(a) =
                Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-")
            else {
                unreachable!()
            };
            a
        },
        base64::engine::general_purpose::NO_PAD,
    );

    let mut output_buf = [0u8; 6];

    CACHE_B64_ENGINE
        .encode_slice(search_id, &mut output_buf)
        .unwrap();

    CACHE_NAMESPACE.to_owned()
        + &query.replace(' ', "-")
        + ":"
        + str::from_utf8(&output_buf).unwrap()
        + ":"
}
