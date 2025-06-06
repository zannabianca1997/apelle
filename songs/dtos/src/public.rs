use chrono::{DateTime, Duration, offset::FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
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
    pub source_data: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResolveSongRequest {
    /// URN of the song source
    pub source_urn: String,
    /// Data that the user used to define the song
    ///
    /// e.g. the video id for youtube
    pub data: Value,
}
