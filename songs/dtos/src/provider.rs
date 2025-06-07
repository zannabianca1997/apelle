//! DTOs on the inner queue

use std::collections::HashSet;

use chrono::Duration;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<u16>,
        message: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQueryParams {
    #[serde(rename = "q")]
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResponseItem<D = Value, R = Value> {
    /// Data to pass the frontend describing the song
    pub details: D,
    /// Data to pass the service to resolve the song
    pub state: SearchResponseItemState<R>,
}

/// How to resolve this search item
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "state")]
pub enum SearchResponseItemState<R = Value> {
    /// Need to be resolved
    New { resolve: R },
    /// Is a known song
    Known { id: Uuid },
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
pub enum ResolveResponse<P = Value, C = Value> {
    /// This is a new song, still unregistered
    New {
        /// Title of the song
        title: String,
        /// Duration of the song
        #[serde(with = "apelle_common::iso8601::duration")]
        duration: Duration,
        /// Additional data from the song source to provide the frontend
        #[serde(default, skip_serializing_if = "Option::is_none")]
        public: Option<P>,
        /// Data for the PUT callback when the song entity is generated
        #[serde(default, skip_serializing_if = "Option::is_none")]
        callback: Option<C>,
    },
    /// This is an existing song
    Existing {
        /// Title of the song
        id: Uuid,
        /// Additional data from the song source to provide the frontend
        #[serde(default, skip_serializing_if = "Option::is_none")]
        public: Option<P>,
    },
}

/// Path parameters for the songs endpoint
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SongsPathParams {
    pub id: Uuid,
}
