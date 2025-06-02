//! DTOs on the inner queue

use std::collections::HashSet;

use chrono::Duration;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use url::Url;

/// Register himself as a provider
#[derive(Debug, Clone, Deserialize)]
pub struct ProviderRegistration {
    /// URNs of the sources this provider can answer for
    pub source_urns: HashSet<String>,
    /// Url where this provider is serving the provider API
    pub url: Url,
    /// Use fast handshake
    ///
    /// This suggest to the `songs` service that the webhook is known to work
    /// and that checks can be skipped
    pub fast_handshake: bool,
}

/// Register himself as a provider
#[derive(Debug, Clone, Serialize)]
pub struct ProviderRegistrationRef<'a> {
    /// URNs of the sources this provider can answer for
    pub source_urns: &'a [&'a str],
    /// Url where this provider is serving the provider API
    pub url: &'a Url,
    /// Use fast handshake
    ///
    /// This suggest to the `songs` service that the webhook is known to work
    /// and that checks can be skipped
    pub fast_handshake: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "error")]
pub enum ProviderRegistrationError {
    /// Registration failed as no sources were provided
    NoSources,
    /// Registration failed as `songs` does not know some of the urns provided
    UnknownSources { urns: HashSet<String> },
    /// Registration failed as the webhook endpoint answered with a non 2xx
    /// status code to a GET request
    WebhookFailed {
        status: Option<u16>,
        message: String,
    },
}

/// Signal that a song has been resolved,
/// and provide the corresponding data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse<P = Box<RawValue>, T = Box<RawValue>> {
    /// Title of the song
    pub title: String,
    /// Duration of the song
    pub duration: Duration,
    /// Additional data from the song source to provide the frontend
    pub public: Option<P>,
    /// Data for the PUT callback when the song entity is generated
    pub callback: Option<T>,
}
