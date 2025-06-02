//! DTOs returned by the YouTube API

use std::collections::HashMap;

use chrono::Duration;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "totalResults")]
    pub total_results: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Video {
    pub id: String,
    pub etag: Option<String>,
    pub snippet: Snippet,
    #[serde(rename = "contentDetails")]
    pub content_details: ContentDetails,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Snippet {
    pub title: String,
    #[serde(default)]
    pub thumbnails: HashMap<String, Thumbnail>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Thumbnail {
    pub width: u32,
    pub height: u32,
    pub url: Url,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContentDetails {
    #[serde(with = "apelle_common::iso8601::duration")]
    pub duration: Duration,
}
