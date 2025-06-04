use std::sync::Arc;

use apelle_songs_dtos::{
    provider::{ProviderRegistrationError, ProviderRegistrationRef},
    source::SourceRegisterRef,
};
use axum::{Router, extract::FromRef};
use config::Config;
use futures::{FutureExt, TryFutureExt};
use reqwest::Response;
use snafu::{ResultExt, Snafu};
use sqlx::PgPool;
use tracing::{Instrument, info_span};
use url::Url;

pub mod config;
mod provider;

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {
    DbConnectionError {
        source: sqlx::Error,
    },
    #[snafu(display("Error while connecting to the songs service"))]
    SongsConnectionError {
        source: reqwest::Error,
    },
    #[snafu(display("Connected to the wrong service: expected 'songs', got {name}"))]
    WrongServiceName {
        name: String,
    },
    #[snafu(display("Error while registering as a provider: {source:?}"))]
    ProviderRegistrationError {
        #[snafu(source(false))]
        source: ProviderRegistrationError,
    },
}

#[derive(Debug, Clone, FromRef)]
struct App {
    db: PgPool,
    songs_client: reqwest::Client,
    youtube: Arc<YoutubeApi>,
}

#[derive(Debug, Clone)]
struct YoutubeApi {
    pub api_key: String,
    pub api_search_url: Url,
    pub api_list_url: Url,

    pub public_url: Url,
}

const YOUTUBE_SOURCE_URN: &str = "urn:apelle:sources:youtube";

const YOUTUBE_SOURCE: SourceRegisterRef<'static> = SourceRegisterRef {
    urn: YOUTUBE_SOURCE_URN,
    name: "YouTube",
};

pub async fn app(
    Config {
        songs_url,
        self_url,
        fast_handshake,
        skip_source_registration,
        youtube,
        db_url,
    }: Config,
) -> Result<(Router, impl AsyncFnOnce() -> Result<(), MainError>), MainError> {
    tracing::info!("Connecting to database");
    let db = PgPool::connect(db_url.as_str())
        .map(|r| r.context(DbConnectionSnafu))
        .instrument(info_span!("Connecting to database"));

    tracing::info!("Connecting to songs service");
    let songs_client = reqwest::Client::new();

    let handshake = async {
        if !fast_handshake {
            // Check that the url is correct and point to the songs service
            let connected_service_name = songs_client
                .get(songs_url.join("/service/name").unwrap())
                .send()
                .map(|r| r.and_then(Response::error_for_status))
                .and_then(Response::text)
                .await
                .context(SongsConnectionSnafu)?;
            if connected_service_name != "songs" {
                return Err(MainError::WrongServiceName {
                    name: connected_service_name,
                });
            }
        }

        if !skip_source_registration {
            // Register the youtube source if not registered already
            // We care only that this request succeeds
            tracing::info!(
                urn = YOUTUBE_SOURCE_URN,
                name = YOUTUBE_SOURCE.name,
                "Registering youtube source"
            );
            songs_client
                .post(songs_url.join("/sources").unwrap())
                .json(&YOUTUBE_SOURCE)
                .send()
                .await
                .and_then(Response::error_for_status)
                .context(SongsConnectionSnafu)?;
        };

        Ok(())
    };

    let (db, _) = tokio::try_join!(db, handshake)?;

    let youtube = YoutubeApi {
        api_key: youtube.api_key,
        api_search_url: youtube
            .api_search_url
            .unwrap_or_else(|| youtube.api_url.join("search").unwrap()),
        api_list_url: youtube
            .api_list_url
            .unwrap_or_else(|| youtube.api_url.join("videos").unwrap()),
        public_url: youtube.public_url,
    };

    Ok((
        Router::new()
            .nest("/provider", provider::provider())
            .with_state(App {
                db,
                songs_client: songs_client.clone(),
                youtube: Arc::new(youtube),
            }),
        async move || {
            // Doing this after the server started up so the web hook can answer
            // to the song service

            // Registering us as a provider
            let url = self_url.join("/provider").unwrap();
            tracing::info!(
                source_urn = YOUTUBE_SOURCE_URN,
                %url,
                "Registering as a provider"
            );
            let r = songs_client
                .post(songs_url.join("/providers").unwrap())
                .json(&ProviderRegistrationRef {
                    source_urns: &[YOUTUBE_SOURCE_URN],
                    url: &url,
                    fast_handshake,
                })
                .send()
                .await
                .context(SongsConnectionSnafu)?;
            if !r.status().is_success() {
                if !r.status().is_client_error() {
                    r.error_for_status().context(SongsConnectionSnafu)?;
                    unreachable!()
                }
                let source = r.json().await.context(SongsConnectionSnafu)?;
                return Err(MainError::ProviderRegistrationError { source });
            }

            Ok(())
        },
    ))
}
