use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, hash_map::Entry},
    hash::{DefaultHasher, Hash, Hasher},
};

use apelle_common::{
    Reporter, ServicesClient,
    common_errors::{CacheError, CacheSnafu},
    db::{SqlError, SqlTx},
    normalize_query,
    paginated::{PageInfo, Paginated, PaginationParams},
};
use axum::{
    Json, debug_handler,
    extract::{Query, State},
    response::IntoResponse,
};
use base64::{Engine, alphabet::Alphabet, engine::GeneralPurpose};
use const_format::concatcp;
use futures::{FutureExt as _, TryFutureExt as _, TryStreamExt, stream::FuturesUnordered};
use rand::{SeedableRng, seq::IndexedRandom};
use rand_xoshiro::Xoshiro256PlusPlus;
use redis::{AsyncCommands, aio::ConnectionManager};
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use snafu::{ResultExt, Snafu};
use url::Url;

use apelle_songs_dtos::{
    provider::{self, SearchQueryParamsRef},
    public::{SearchQueryParams, SearchResponseItem},
};
use utoipa::{IntoResponses, openapi};

use crate::{
    ProvidersConfig,
    providers::{provider_for_urn, search_endpoint},
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
    RequestError {
        upstream: Url,
        source: reqwest::Error,
    },
    #[snafu(display("Failed to deserialize json present in the cache"))]
    CacheJsonError {
        source: serde_json::Error,
    },

    #[snafu(display("Bad page token: query not made before"))]
    BadPage,
    UnknownSources {
        missing: HashSet<String>,
    },
    NoSources,
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
            SearchError::RequestError { source, upstream } => {
                tracing::error!(
                    %upstream,
                    "Request to the upstream provider failed: {}",
                    Reporter(source)
                );
                StatusCode::BAD_GATEWAY.into_response()
            }
            SearchError::BadPage => StatusCode::BAD_REQUEST.into_response(),
            SearchError::UnknownSources { missing } => {
                (StatusCode::NOT_FOUND, Json(missing)).into_response()
            }
            SearchError::NoSources => {
                tracing::error!("No sources registered");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

impl IntoResponses for SearchError {
    fn responses() -> BTreeMap<String, utoipa::openapi::RefOr<utoipa::openapi::response::Response>>
    {
        [
            (
                StatusCode::INTERNAL_SERVER_ERROR.as_str().to_owned(),
                openapi::RefOr::T(openapi::Response::new("Internal Server Error")),
            ),
            (
                StatusCode::BAD_REQUEST.as_str().to_owned(),
                openapi::RefOr::T(openapi::Response::new(
                    "Bad page token: either invalid, not from this search, or expired",
                )),
            ),
            (
                StatusCode::BAD_GATEWAY.as_str().to_owned(),
                openapi::RefOr::T(openapi::Response::new(
                    "Error in connectiong to the providers",
                )),
            ),
            (
                StatusCode::NOT_FOUND.as_str().to_owned(),
                openapi::RefOr::T(openapi::Response::new(
                    "One of the requested sources was not found",
                )),
            ),
        ]
        .into_iter()
        .chain(SqlError::responses())
        .chain(CacheError::responses())
        .collect()
    }
}

mod cursor;
use cursor::Cursor;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchState {
    items: Vec<SearchResponseItem>,
    continuations: HashMap<String, Paginated<provider::SearchResponseItem, Value>>,
    rng: Xoshiro256PlusPlus,
}

/// Search for songs
///
/// Search for songs based on a query. The query will be normalized by
/// lowercasing it, removing all the non-alphanumeric characters, and
/// normalizing all spaces with a single space.
///
/// If songs from only one source is wanted, the `source` query parameter can be
/// used. It can be repeated to generate an aggregated query. If omitted, all
/// available sources will be used.
///
/// The order of the songs from a single source is kept. Different sources are
/// randomly mixed, but in a way that depends only from the set of sources
/// choosen and the query.
///
/// The songs are paginated. The cursor is an opaque string. The duration of
/// validity of the cursor depends on the specific providers. In case it
/// expires, a `400 Bad Request` is returned.
#[debug_handler(state=crate::App)]
#[utoipa::path(
    get,
    path = "/search",
    responses(
        (status = StatusCode::OK, description = "Search results", body = Paginated<SearchResponseItem, Cursor>),
        SearchError
    ),
    params(
        SearchQueryParams,
        PaginationParams<Cursor>
    )
)]
pub async fn search(
    State(mut cache): State<ConnectionManager>,
    mut tx: SqlTx,
    State(ProvidersConfig {
        cache_expiration, ..
    }): State<ProvidersConfig>,
    client: ServicesClient,
    axum_extra::extract::Query(SearchQueryParams {
        query: original_query,
        sources,
    }): axum_extra::extract::Query<SearchQueryParams>,
    Query(PaginationParams { page, page_size }): Query<PaginationParams<Cursor>>,
) -> Result<Json<Paginated<SearchResponseItem, Cursor>>, SearchError> {
    // Normalize the query
    let query = normalize_query(&original_query);

    // Fetch the providers
    let providers = async {
        let mut qb = sqlx::QueryBuilder::new("SELECT urn FROM source");
        if !sources.is_empty() {
            // Using only the requested sources
            let collect = sources.iter().map(String::as_str).collect::<Vec<_>>();
            qb.push(" WHERE urn = ANY(")
                .push_bind(collect)
                .push("::text[])");
        }
        // Sources we found in the db
        let mut missing: HashSet<&str> = sources.iter().map(String::as_str).collect();
        let providers = qb
            .build_query_scalar::<String>()
            .fetch(&mut tx)
            // Keep track of the seen sources
            .inspect_ok(|s| {
                missing.remove(s.as_str());
            })
            .map_err(|source| SearchError::from(SqlError::from(source)))
            // Find a provider for each source
            .map_ok(|urn| {
                provider_for_urn(cache.clone(), urn.clone())
                    .map_ok(|provider| (urn, provider))
                    .map_err(SearchError::from)
            })
            .try_collect::<FuturesUnordered<_>>()
            .await?;

        if !missing.is_empty() {
            // Some sources are unknow
            return Err(SearchError::UnknownSources {
                missing: missing.into_iter().map(ToOwned::to_owned).collect(),
            });
        }

        // Actually find the providers now we know all sources are accounted for
        providers.try_collect::<BTreeMap<_, _>>().await
    };

    // Get the search info, if existing
    let search_info = async {
        let mut hasher = DefaultHasher::new();
        query.hash(&mut hasher);
        BTreeSet::from_iter(&sources).hash(&mut hasher);
        let search_hash = hasher.finish();

        let cache_key = cache_key(&query, search_hash);

        Ok((
            cache
                .clone()
                .get::<_, Option<String>>(&cache_key)
                .await
                .context(CacheSnafu)?
                .map(|s| serde_json::from_str(&s))
                .transpose()
                .context(CacheJsonSnafu)?
                .unwrap_or_else(|| SearchState {
                    items: Vec::new(),
                    continuations: HashMap::new(),
                    rng: Xoshiro256PlusPlus::seed_from_u64(search_hash),
                }),
            cache_key,
        ))
    };

    let (providers, (mut search_info, cache_key)) = tokio::try_join!(providers, search_info)?;
    if providers.is_empty() {
        return Err(SearchError::NoSources);
    }

    let SearchState {
        items,
        continuations,
        rng,
    } = &mut search_info;

    let Cursor { offset } = page.unwrap_or_else(Cursor::new);
    let end = offset + page_size;

    while end as usize > items.len() {
        let Some((source, provider)) = providers
            .iter()
            .filter(|(source, _)| {
                !continuations
                    .get(*source)
                    .is_some_and(|p| p.items.is_empty() && p.page_info.next.is_none())
            })
            .collect::<Vec<_>>()
            .choose(rng)
            .copied()
        else {
            // All sources are exhausted
            break;
        };

        let continuation = match continuations.entry(source.clone()) {
            Entry::Occupied(mut occupied_entry) => {
                // Fetch the next page if needed
                let Paginated {
                    items,
                    page_info: PageInfo { next, .. },
                } = occupied_entry.get();

                if items.is_empty() {
                    let next = next
                        .clone()
                        .expect("The exausted source should have been filtered out");
                    let mut continuation: Paginated<_, _> = client
                        .get(search_endpoint(provider))
                        .query(&SearchQueryParamsRef {
                            query: &original_query,
                        })
                        .query(&PaginationParams {
                            page_size,
                            page: Some(next),
                        })
                        .send()
                        .map(|r| r.and_then(Response::error_for_status))
                        .and_then(|r| r.json())
                        .await
                        .with_context(|_| RequestSnafu {
                            upstream: provider.clone(),
                        })?;

                    continuation.items.reverse();

                    occupied_entry.insert(continuation);
                }

                occupied_entry
            }
            Entry::Vacant(vacant_entry) => {
                // Ask the provider for the first page
                let mut continuation: Paginated<_, _> = client
                    .get(search_endpoint(provider))
                    .query(&SearchQueryParamsRef {
                        query: &original_query,
                    })
                    .send()
                    .map(|r| r.and_then(Response::error_for_status))
                    .and_then(|r| r.json())
                    .await
                    .with_context(|_| RequestSnafu {
                        upstream: provider.clone(),
                    })?;

                continuation.items.reverse();

                vacant_entry.insert_entry(continuation)
            }
        }
        // Get the next item from the continuation
        .get_mut()
        .items
        .pop();

        let Some(provider::SearchResponseItem { details, state }) = continuation else {
            // Empty page from provider ~ weird
            tracing::warn!(source, %provider, "Empty page from provider");
            continue;
        };

        items.push(SearchResponseItem {
            source: source.clone(),
            details,
            state,
        });
    }

    let total = continuations
        .iter()
        .map(|(_, p)| p.page_info.total)
        .reduce(|a, b| a.and_then(|a| b.map(|b| a.saturating_add(b))))
        .flatten();

    let items = items[offset as usize..end as usize].to_vec();

    // Update the cache with the extended state
    // Note that as the extension is stable, we can freely overwrite
    let _: () = cache
        .set_ex(
            cache_key,
            serde_json::to_string(&search_info).expect("Serialization should be infallible"),
            cache_expiration.num_seconds() as _,
        )
        .await
        .context(CacheSnafu)?;

    let page_size = items.len() as u32;
    let end = offset + page_size;
    Ok(Json(Paginated {
        page_info: PageInfo {
            size: page_size,
            total,
            first: Some(Cursor::new()),
            prev: (offset > 0).then_some(Cursor {
                offset: offset.saturating_sub(page_size),
            }),
            page: Cursor { offset },
            next: total
                .is_none_or(|total| end < total)
                .then_some(Cursor { offset: end }),
            // Avoid giving the last page as it means runnign over all the
            last: None,
        },
        items,
    }))
}

fn cache_key(query: &str, search_hash: u64) -> String {
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

    let mut output_buf = [0u8; 12];

    CACHE_B64_ENGINE
        .encode_slice(search_hash.to_le_bytes(), &mut output_buf)
        .unwrap();

    CACHE_NAMESPACE.to_owned()
        + &query.replace(' ', "-")
        + ":"
        + str::from_utf8(&output_buf).unwrap()
}
