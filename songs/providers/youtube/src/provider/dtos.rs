use serde::{Deserialize, Serialize};
use url::Url;

pub mod youtube;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResolveRequest {
    pub video_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicSongData {
    pub video_id: String,
    pub url: Url,
    pub thumbs: Vec<Thumbnail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    pub width: u32,
    pub height: u32,
    pub url: Url,
}

impl From<youtube::Thumbnail> for Thumbnail {
    fn from(youtube::Thumbnail { width, height, url }: youtube::Thumbnail) -> Self {
        Self { width, height, url }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchItemDetails {
    pub title: String,
    pub url: Url,
    pub thumbnails: Vec<Thumbnail>,
}

pub type SearchResponseItem =
    apelle_songs_dtos::provider::SearchResponseItem<SearchItemDetails, ResolveRequest>;
