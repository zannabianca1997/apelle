use apelle_songs_dtos::{
    provider::{ProviderRegistrationError, ProviderRegistrationRef},
    source::SourceRegisterRef,
};
use axum::{Router, extract::FromRef, response::NoContent, routing::get};
use config::Config;
use futures::{FutureExt, TryFutureExt};
use reqwest::Response;
use snafu::{ResultExt, Snafu};

pub mod config;

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {
    #[snafu(display("Error while connecting to the songs service"))]
    SongsConnectionError { source: reqwest::Error },
    #[snafu(display("Connected to the wrong service: expected 'songs', got {name}"))]
    WrongServiceName { name: String },
    #[snafu(display("Error while registering as a provider: {source:?}"))]
    ProviderRegistrationError {
        #[snafu(source(false))]
        source: ProviderRegistrationError,
    },
}

#[derive(Debug, Clone, FromRef)]
struct App {
    songs_client: reqwest::Client,
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
    }: Config,
) -> Result<(Router, impl AsyncFnOnce() -> Result<(), MainError>), MainError> {
    tracing::info!("Connecting to songs service");
    let songs_client = reqwest::Client::new();

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

    Ok((
        Router::new()
            .route("/", get(async || NoContent))
            .with_state(App {
                songs_client: songs_client.clone(),
            }),
        async move || {
            // Doing this after the server started up so the web hook
            // answers to the song service

            // Registering us as a provider
            tracing::info!(
                source_urn = YOUTUBE_SOURCE_URN,
                url =% self_url,
                "Registering as a provider"
            );
            let r = songs_client
                .post(songs_url.join("/providers").unwrap())
                .json(&ProviderRegistrationRef {
                    source_urns: &[YOUTUBE_SOURCE_URN],
                    url: &self_url,
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
