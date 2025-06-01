//! DTOs on the inner queue

use chrono::Duration;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use uuid::Uuid;

const REQUESTS_REDIS_PREFIX: &str = "songs::song-requests:";
const DATA_REQUESTS_REDIS_PREFIX: &str = "songs::song-data-requests:";

/// Request for a song to be resolved
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongRequest<T = Box<RawValue>> {
    /// Data that the user used to define the song
    ///
    /// This is significant only for the source
    pub data: T,
    /// If source data should be included in the response
    pub source_data: bool,
    /// Callback channel
    ///
    /// When someone resolve this request, it should post a [`SongResponse`]
    /// to this channel
    pub callback_channel: String,
}

impl<T> SongRequest<T> {
    /// Redis key of the request queue
    pub fn redis_key(source_urn: &str) -> String {
        let mut res = String::with_capacity(REQUESTS_REDIS_PREFIX.len() + source_urn.len());
        res.push_str(REQUESTS_REDIS_PREFIX);
        res.push_str(source_urn);
        return res;
    }
}

/// Signal that a song has been resolved,
/// and provide the corresponding data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongResponse<T = Box<RawValue>> {
    /// Title of the song
    pub title: String,
    /// Duration of the song
    pub duration: Duration,
    /// Additional data from the song source
    pub source_data: Option<T>,
    /// Callback channel
    ///
    /// When the song entity has been created, a message
    /// of the type [`SongAdded`] will be published to
    /// this channel so the source can store data
    /// referencing the song
    pub callback_channel: Option<String>,
}

/// Data about a song that has been added
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongAdded {
    pub id: Uuid,
}

/// Request for the data about a known song
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongDataRequest {
    pub song_id: Uuid,
    pub callback_channel: String,
}

impl SongDataRequest {
    /// Redis key of the request queue
    pub fn redis_key(source_urn: &str) -> String {
        let mut res = String::with_capacity(DATA_REQUESTS_REDIS_PREFIX.len() + source_urn.len());
        res.push_str(DATA_REQUESTS_REDIS_PREFIX);
        res.push_str(source_urn);
        return res;
    }
}
