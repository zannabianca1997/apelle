use axum::{Router, extract::FromRef, routing::get};
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
}

#[derive(Debug, Clone, FromRef)]
struct App {
    songs_client: reqwest::Client,
}

pub async fn app(
    Config {
        songs_url,
        self_url,
    }: Config,
) -> Result<Router, MainError> {
    tracing::info!("Connecting to songs service");
    let songs_client = reqwest::Client::new();

    // Check that the url is correct and point to the songs service
    let name = songs_client
        .get(songs_url.join("service/name").unwrap())
        .send()
        .map(|r| r.and_then(Response::error_for_status))
        .and_then(Response::text)
        .await
        .context(SongsConnectionSnafu)?;
    if name != "songs" {
        return Err(MainError::WrongServiceName { name });
    }

    Ok(Router::new().with_state(App { songs_client }))
}
