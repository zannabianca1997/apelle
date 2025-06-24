use std::borrow::{Borrow, BorrowMut};

use apelle_common::{
    TracingClient,
    common_errors::{CacheError, CacheSnafu, SQLError, SQLSnafu},
};
use apelle_songs_dtos::provider::{
    ProviderRegistration, ProviderRegistrationError as ProviderRegistrationErrorDto,
};
use axum::{
    Json, debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, NoContent},
};
use const_format::concatcp;
use futures::FutureExt;
use redis::{AsyncCommands as _, aio::ConnectionManager};
use reqwest::Response;
use snafu::{ResultExt, Snafu};
use sqlx::PgPool;
use textwrap_macros::unfill;
use url::Url;
use uuid::Uuid;

use crate::{CACHE_NAMESPACE, ProvidersConfig, seen_sources::SeenSourcesWorker};

const PROVIDERS_NAMESPACE: &str = concatcp!(CACHE_NAMESPACE, "providers:");

pub fn providers_set_cache_key(mut urn: &str) -> String {
    urn = urn.trim_start_matches("urn:");

    let mut key = String::with_capacity(PROVIDERS_NAMESPACE.len() + urn.len());
    key += PROVIDERS_NAMESPACE;
    key += urn;

    key
}

#[derive(Debug, Snafu)]
pub enum ProviderRegistrationError {
    #[snafu(transparent)]
    SQLError {
        source: SQLError,
    },
    #[snafu(transparent)]
    CacheError {
        source: CacheError,
    },
    UnknownSource {
        urn: String,
    },
    WebhookFailed {
        source: reqwest::Error,
    },
}
impl IntoResponse for ProviderRegistrationError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::BAD_REQUEST,
            Json(match self {
                ProviderRegistrationError::SQLError { source } => return source.into_response(),
                ProviderRegistrationError::CacheError { source } => return source.into_response(),
                ProviderRegistrationError::UnknownSource { urn } => {
                    ProviderRegistrationErrorDto::UnknownSource { urn }
                }
                ProviderRegistrationError::WebhookFailed { source } => {
                    ProviderRegistrationErrorDto::WebhookFailed {
                        status: source.status().map(|s| s.as_u16()),
                        message: source.to_string(),
                    }
                }
            }),
        )
            .into_response()
    }
}

/// Register a new provider
///
/// This will register a new provider for the specific song. The sources must be
/// already registered.
///
/// If `fast_handshake` is false, the provider must be already running as it
/// will be checked.
#[debug_handler(state=crate::App)]
#[utoipa::path(
    post,
    path = "/providers",
    responses(
        (status = StatusCode::NO_CONTENT, description = "Provider registered")
    )
)]
pub async fn register(
    State(db): State<PgPool>,
    client: TracingClient,
    State(mut cache): State<redis::aio::ConnectionManager>,
    State(ProvidersConfig {
        honor_fast_handshake,
        ..
    }): State<ProvidersConfig>,
    State(seen_sources): State<SeenSourcesWorker>,
    Json(ProviderRegistration {
        source_urn,
        url,
        fast_handshake,
    }): Json<ProviderRegistration>,
) -> Result<NoContent, ProviderRegistrationError> {
    // Check that all the sources are registered
    // and that the webhook is reachable
    if honor_fast_handshake && fast_handshake {
        // Only check that the urn is known
        check_urn_presence(&db, &source_urn).await?;
    } else {
        // Full handshake
        tokio::try_join!(
            check_urn_presence(&db, &source_urn),
            check_webhook(&client, &url)
        )?;
    }

    // Marking that we seen a provider for the sources and registering the
    // webhook as a provider

    let _: ((), ()) = tokio::try_join!(
        seen_sources.seen_urn(source_urn.clone()).map(Ok),
        cache
            .sadd(providers_set_cache_key(&source_urn), url.to_string())
            .map(|r| r.context(CacheSnafu))
    )?;

    Ok(NoContent)
}

/// Check that the source is registered
async fn check_urn_presence(db: &PgPool, urn: &str) -> Result<(), ProviderRegistrationError> {
    let exist: bool = sqlx::query_scalar(
        unfill!(
            "
            SELECT EXISTS (
                SELECT 1
                FROM source
                WHERE source.urn = $1
            )
            "
        )
        .trim_ascii(),
    )
    .bind(urn)
    .fetch_one(db)
    .await
    .context(SQLSnafu)?;

    if !exist {
        return Err(ProviderRegistrationError::UnknownSource {
            urn: urn.to_owned(),
        });
    }

    Ok(())
}

/// Check that the webhook is reachable
///
/// We leverage the fact that the provider API requires the
/// root to return a 2xx on a GET request
async fn check_webhook(client: &TracingClient, url: &Url) -> Result<(), ProviderRegistrationError> {
    client
        .get(url.clone())
        .send()
        .await
        .and_then(Response::error_for_status)
        .context(WebhookFailedSnafu)
        .map(|_| ())
}

/// Get a random provider for the given urn
pub async fn provider_for_urn(
    mut cache: impl BorrowMut<ConnectionManager>,
    urn: impl Borrow<str>,
) -> Result<Url, CacheError> {
    let cache_key = providers_set_cache_key(urn.borrow());
    let cache = cache.borrow_mut();
    Ok(loop {
        // Take a random provider from the registered ones
        let provider = cache
            .srandmember::<_, String>(&cache_key)
            .await
            .context(CacheSnafu)?;
        let Ok(provider) = Url::parse(&provider) else {
            tracing::error!(provider, "Invalid provider url");
            // Removing the bad url
            cache
                .srem::<_, _, i64>(&cache_key, provider)
                .await
                .context(CacheSnafu)?;
            continue;
        };
        break provider;
    })
}

pub fn resolve_endpoint(provider: &Url) -> Url {
    let mut provider = provider.clone();
    provider.path_segments_mut().unwrap().push("resolve");
    provider
}

pub fn solved_endpoint(provider: &Url, id: Uuid) -> Url {
    let mut provider = provider.clone();
    provider
        .path_segments_mut()
        .unwrap()
        .push("solved")
        .push(&id.to_string());
    provider
}

pub fn search_endpoint(provider: &Url) -> Url {
    let mut provider = provider.clone();
    provider.path_segments_mut().unwrap().push("search");
    provider
}
