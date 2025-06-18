//! DTOs returned by the YouTube API

use std::collections::HashMap;

use chrono::Duration;
use serde::{Deserialize, Serialize};
use url::Url;

// == Pagination ==

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
    #[serde(
        rename = "prevPageToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prev_page_token: Option<String>,
    #[serde(
        rename = "nextPageToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page_token: Option<String>,
}

impl<T> IntoIterator for Paginated<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Paginated<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageInfo {
    #[serde(rename = "totalResults")]
    pub total_results: u32,
}

// == Video ==

#[derive(Debug, Clone, Deserialize)]
pub struct Video {
    // pub id: String,
    pub etag: Option<String>,
    pub snippet: Snippet,
    #[serde(rename = "contentDetails")]
    pub content_details: ContentDetails,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

// == Search ==

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResult {
    pub id: SearchResultId,
    pub snippet: Snippet,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "kind")]
pub enum SearchResultId {
    #[serde(rename = "youtube#video")]
    Video {
        #[serde(rename = "videoId")]
        video_id: String,
    },
    #[serde(rename = "youtube#playlist")]
    Playlist {
        // #[serde(rename = "playlistId")]
        // playlist_id: String,
    },
    #[serde(rename = "youtube#channel")]
    Channel {
        // #[serde(rename = "channelId")]
        // channel_id: String,
    },
}
