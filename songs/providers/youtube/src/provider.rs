use std::collections::HashMap;

use apelle_songs_dtos::provider::SongsPathParams;
use axum::{
    Router, debug_handler,
    http::HeaderName,
    response::NoContent,
    routing::{get, post, put},
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use url::Url;

use dtos::youtube::Thumbnail;

use crate::App;

mod dtos;

const GOOGLE_API_KEY_HEADER: HeaderName = HeaderName::from_static("x-goog-api-key");

#[derive(Debug, Serialize, Deserialize)]
struct YoutubeSongData {
    video_id: String,
    etag: Option<String>,
    fetched: DateTime<FixedOffset>,
    thumbs: HashMap<String, Thumbnail>,
}

fn video_url(public_url: &Url, video_id: &str) -> Url {
    let mut url = public_url.clone();
    url.query_pairs_mut().append_pair("v", video_id);
    url
}

#[debug_handler(state=crate::App)]
async fn ping() -> NoContent {
    tracing::debug!("Pinged on the provider interface");
    NoContent
}

mod delete;
mod get;
mod insert;
mod resolve;

pub fn provider() -> Router<App> {
    Router::new()
        .route("/", get(ping))
        .route("/resolve", post(resolve::resolve))
        .route(
            "/solved/{id}",
            put(insert::insert).get(get::get).delete(delete::delete),
        )
}
