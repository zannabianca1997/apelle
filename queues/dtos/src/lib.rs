use apelle_configs_dtos::QueueConfigCreate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

pub mod events;
pub mod model;

#[derive(Deserialize, Clone, Debug, ToSchema, Default)]
pub struct QueueCreate {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub config: Config,
}

#[derive(Deserialize, Clone, Debug, ToSchema)]
#[serde(untagged)]
pub enum Config {
    Existing(Uuid),
    New(QueueConfigCreate),
}

impl Default for Config {
    fn default() -> Self {
        Self::Existing(Uuid::nil())
    }
}

#[derive(Deserialize, Serialize, IntoParams)]
pub struct GetQueryParams {
    /// Return the full queue config instead of just the UUID
    #[serde(default)]
    pub config: bool,
    /// Return the full song data instead of just the UUID
    #[serde(default)]
    pub songs: bool,
    // For each song, return the source data in addition to the song data (like
    // thumbnails or public url)
    #[serde(default)]
    pub songs_source: bool,
}

pub type PushSyncEventQueryParam = GetQueryParams;
