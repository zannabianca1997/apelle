use std::collections::HashSet;

use apelle_common::common_errors::{SQLError, SQLSnafu};
use apelle_songs_dtos::provider::{
    ProviderRegistration, ProviderRegistrationError as ProviderRegistrationErrorDto,
};
use axum::{
    Json, debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, NoContent},
};
use reqwest::Response;
use snafu::{ResultExt, Snafu};
use sqlx::PgPool;
use url::Url;

#[derive(Debug, Snafu)]
pub enum ProviderRegistrationError {
    #[snafu(transparent)]
    SQLError {
        source: SQLError,
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
    State(client): State<reqwest::Client>,
    Json(ProviderRegistration { source_urns, url }): Json<ProviderRegistration>,
) -> Result<NoContent, ProviderRegistrationError> {
    // Check that all the sources are registered
    // and that the webhook is reachable
    tokio::try_join!(
        check_urn_presence(&db, &source_urns),
        check_webhook(&client, &url)
    )?;

    // Marking that we seen a provider for the sources
    set_sources_as_seen(&db, &source_urns)
        .await
        .context(SQLSnafu)?;

    Ok(NoContent)
}

// Check that all sources are registered
async fn check_urn_presence(
    db: &PgPool,
    urns: &HashSet<String>,
) -> Result<(), ProviderRegistrationError> {
    if urns.is_empty() {
        return Err(ProviderRegistrationError::NoSources);
    }

    if urns.len() == 1 {
        let urn = urns.iter().next().unwrap();

        let count: i64 = sqlx::query_scalar(concat!(
            "SELECT COUNT(*) ",
            "FROM source ",
            "WHERE urn = $1"
        ))
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

// Check that all sources are registered
async fn set_sources_as_seen(db: &PgPool, urns: &HashSet<String>) -> Result<(), sqlx::Error> {
    if urns.len() == 1 {
        let urn = urns.iter().next().unwrap();

        sqlx::query(concat!(
            "UPDATE source ",
            "SET last_heard = NOW() ",
            "WHERE urn = $1"
        ))
        .bind(urn)
        .execute(db)
        .await?;

        return Ok(());
    }

    let mut qb = sqlx::QueryBuilder::new(concat!(
        "UPDATE source ",
        "SET last_heard = NOW() ",
        "WHERE urn = ANY("
    ));
    let mut sep = qb.separated(", ");
    for urn in urns {
        sep.push_bind(urn);
    }
    qb.push(")");
    qb.build().execute(db).await?;

    Ok(())
}

async fn check_webhook(
    client: &reqwest::Client,
    url: &Url,
) -> Result<(), ProviderRegistrationError> {
    client
        .get(url.clone())
        .send()
        .await
        .and_then(Response::error_for_status)
        .context(WebhookFailedSnafu)
        .map(|_| ())
}
