use std::collections::HashSet;

use chrono::{DateTime, Duration, offset::FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema, Deserialize)]
pub struct Song {
    /// Unique id of the song
    pub id: Uuid,
    /// Title of the song
    pub title: String,
    /// Duration of the song
    #[serde(with = "apelle_common::iso8601::duration")]
    pub duration: Duration,
    /// User that first added the song
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_by: Option<Uuid>,
    /// When the song was added
    pub created: DateTime<FixedOffset>,
    /// Additional data from the song source
    ///
    /// These contains stuff like thumbnails,
    /// the artist, an url or any source specific
    /// data. They are provided on-demand
    /// as they require querying the source service
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Object, nullable, required = false)]
    pub source_data: Option<Value>,
}

#[derive(Debug, Deserialize, IntoParams, Serialize)]
pub struct SolvedQueryParams {
    #[serde(default = "default_true")]
    /// Include the data from the song source
    pub source_data: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize, ToSchema, Serialize)]
pub struct ResolveSongRequest {
    /// URN of the song source
    pub source: String,
    /// Data that the user used to define the song
    ///
    /// e.g. the video id for youtube
    #[schema(value_type = Object)]
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct SearchQueryParams {
    /// Search query
    #[serde(rename = "q")]
    pub query: String,
    /// List of sources to search
    ///
    /// Empty to use all sources
    #[serde(default, rename = "source")]
    #[into_params(explode)]
    pub sources: HashSet<String>,
}

/// How to resolve this search item
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(tag = "state")]
pub enum SearchResponseItemState<R = Value> {
    /// Need to be resolved
    New {
        #[schema(value_type = Object)]
        data: R,
    },
    /// Is a known song
    Known { id: Uuid },
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchResponseItem<D = Value, R = Value> {
    /// Source that provided this search result
    pub source: String,
    /// Data to pass the frontend describing the song
    #[schema(value_type = Object)]
    pub details: D,
    /// Data to pass the service to resolve the song
    pub state: SearchResponseItemState<R>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UnknownSources(pub Vec<String>);
