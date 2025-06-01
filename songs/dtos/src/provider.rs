//! DTOs on the inner queue

use std::collections::HashSet;

use chrono::Duration;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use url::Url;

/// Register himself as a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRegistration {
    /// URNs of the sources this provider can answer for
    pub source_urn: HashSet<String>,
    /// Url where this provider is serving the provider API
    pub url: Url,
}

/// Registration failed as `songs` does not know some
/// of the urns provided
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownSources {
    pub urns: HashSet<String>,
}

/// Registration failed as the webhook endpoint answered with a
/// non 2xx status code to a GET request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookFailed {
    pub status: u16,
}

/// Signal that a song has been resolved,
/// and provide the corresponding data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse<P = Box<RawValue>, T = Box<RawValue>> {
    /// Title of the song
    pub title: String,
    /// Duration of the song
    pub duration: Duration,
    /// Additional data from the song source
    /// to provide the frontend
    pub public: Option<P>,
    /// Data for the PUT callback when the song
    /// entity is generated
    pub callback: Option<T>,
}
