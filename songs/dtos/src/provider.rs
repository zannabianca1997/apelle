//! DTOs on the inner queue

use std::collections::HashSet;

use chrono::Duration;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use url::Url;
use uuid::Uuid;

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
    #[serde(default)]
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
    #[serde(skip_serializing_if = "std::ops::Not::not")]
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

#[derive(Debug, Clone, Deserialize)]
pub struct ResolveQueryParams {
    #[serde(default)]
    pub public: bool,
}

/// Signal that a song has been resolved,
/// and provide the corresponding data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "state")]
pub enum ResolveResponse<P = Box<RawValue>, C = Box<RawValue>> {
    /// This is a new song, still unregistered
    New {
        /// Title of the song
        title: String,
        /// Duration of the song
        #[serde(with = "apelle_common::iso8601::duration")]
        duration: Duration,
        /// Additional data from the song source to provide the frontend
        public: Option<P>,
        /// Data for the PUT callback when the song entity is generated
        callback: Option<C>,
    },
    /// This is an existing song
    Existing {
        /// Title of the song
        id: Uuid,
        /// Additional data from the song source to provide the frontend
        public: Option<P>,
    },
}

/// Path parameters for the songs endpoint
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SongsPathParams {
    pub id: Uuid,
}
