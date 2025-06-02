use serde::{Deserialize, Serialize};
use url::Url;

pub mod youtube;

#[derive(Debug, Clone, Deserialize)]
pub struct RetrieveRequest {
    pub video_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicSongData {
    pub video_id: String,
    pub url: Url,
    pub thumbs: Vec<Thumbnail>,
}

#[derive(Debug, Clone, Serialize)]
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
