use std::collections::HashSet;

use apelle_common::{
    TracingClient,
    common_errors::{CacheError, SQLError, SQLSnafu},
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
use futures::TryFutureExt as _;
use reqwest::Response;
use snafu::{ResultExt, Snafu};
use sqlx::PgPool;
use url::Url;

use crate::{CACHE_NAMESPACE, FastHandshakeConfig};

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
    NoSources,
    UnknownSources {
        urns: HashSet<String>,
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
                ProviderRegistrationError::UnknownSources { urns } => {
                    ProviderRegistrationErrorDto::UnknownSources { urns }
                }
                ProviderRegistrationError::WebhookFailed { source } => {
                    ProviderRegistrationErrorDto::WebhookFailed {
                        status: source.status().map(|s| s.as_u16()),
                        message: source.to_string(),
                    }
                }
                ProviderRegistrationError::NoSources => ProviderRegistrationErrorDto::NoSources,
            }),
        )
            .into_response()
    }
}

#[debug_handler(state=crate::App)]
pub async fn register(
    State(db): State<PgPool>,
    client: TracingClient,
    State(mut cache): State<redis::aio::ConnectionManager>,
    State(FastHandshakeConfig {
        honor_fast_handshake,
    }): State<FastHandshakeConfig>,
    Json(ProviderRegistration {
        source_urns,
        url,
        fast_handshake,
    }): Json<ProviderRegistration>,
) -> Result<NoContent, ProviderRegistrationError> {
    // Check that all the sources are registered
    // and that the webhook is reachable
    if honor_fast_handshake && fast_handshake {
        // Only check that the urn is known
        check_urn_presence(&db, &source_urns).await?;
    } else {
        // Full handshake
        tokio::try_join!(
            check_urn_presence(&db, &source_urns),
            check_webhook(&client, &url)
        )?;
    }

    // Marking that we seen a provider for the sources
    let set_sources_as_seen = set_sources_as_seen(&db, &source_urns).map_err(|source| {
        ProviderRegistrationError::SQLError {
            source: SQLError { source },
        }
    });

    // Register the webhook as a provider for all the sources
    let mut pipe = redis::pipe();
    for urn in &source_urns {
        pipe.sadd(providers_set_cache_key(&urn), url.to_string());
    }
    let register_provider =
        pipe.exec_async(&mut cache)
            .map_err(|source| ProviderRegistrationError::CacheError {
                source: CacheError { source },
            });

    tokio::try_join!(set_sources_as_seen, register_provider)?;

    Ok(NoContent)
}

/// Check that all sources are registered
async fn check_urn_presence(
    db: &PgPool,
    urns: &HashSet<String>,
) -> Result<(), ProviderRegistrationError> {
    if urns.is_empty() {
        return Err(ProviderRegistrationError::NoSources);
    }

    if urns.len() == 1 {
        let urn = urns.iter().next().unwrap();

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM source WHERE urn = $1")
            .bind(urn)
            .fetch_one(db)
            .await
            .context(SQLSnafu)?;

        if count == 0 {
            return Err(ProviderRegistrationError::UnknownSources { urns: urns.clone() });
        }

        return Ok(());
    }

    let mut qb = sqlx::QueryBuilder::new("SELECT urn FROM source WHERE urn = ANY(");
    let mut sep = qb.separated(", ");
    for urn in urns {
        sep.push_bind(urn);
    }
    qb.push(")");
    let present: HashSet<String> = qb
        .build_query_scalar()
        .fetch_all(db)
        .await
        .context(SQLSnafu)?
        .into_iter()
        .collect();
    if urns != &present {
        return Err(ProviderRegistrationError::UnknownSources {
            urns: urns.difference(&present).cloned().collect(),
        });
    }

    Ok(())
}

/// Marking that we seen a provider for the sources
async fn set_sources_as_seen(db: &PgPool, urns: &HashSet<String>) -> Result<(), sqlx::Error> {
    if urns.len() == 1 {
        let urn = urns.iter().next().unwrap();

        sqlx::query("UPDATE source SET last_heard = NOW() WHERE urn = $1")
            .bind(urn)
            .execute(db)
            .await?;

        return Ok(());
    }

    let mut qb = sqlx::QueryBuilder::new("UPDATE source SET last_heard = NOW() WHERE urn = ANY(");
    let mut sep = qb.separated(", ");
    for urn in urns {
        sep.push_bind(urn);
    }
    qb.push(")");
    qb.build().execute(db).await?;

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
