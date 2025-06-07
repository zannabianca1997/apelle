use apelle_common::{Figment, ProvideDefaults, Provider};
use chrono::Duration;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Url this service is reachable from `songs`
    pub self_url: Url,

    /// Url of the `songs` service
    pub songs_url: Url,

    /// Enable fast handshake
    ///
    /// This won't check the url given, and instruct `songs` to do the same
    pub fast_handshake: bool,

    /// Skip source registration
    ///
    /// This skip the source registration, assuming that `songs` already know
    /// about it (safe to do if this is a replica or runned before)
    pub skip_source_registration: bool,

    /// Youtube api config
    pub youtube: YoutubeConfig,

    /// Database connection string
    pub db_url: Url,
    /// Cache connection string
    pub cache_url: Url,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YoutubeConfig {
    pub api_key: String,

    pub api_url: Option<Url>,
    pub api_search_url: Option<Url>,
    pub api_list_url: Option<Url>,

    pub public_url: Url,

    /// Max number of requests to youtube a single search request can cause
    pub max_upstream_requests: u32,

    /// Size of the page to request from youtube
    pub page_size: u32,

    /// Expiration of the cached searches
    ///
    /// If the search is older than this, it will be refetched from youtube
    #[serde(with = "apelle_common::iso8601::duration")]
    pub expiration: Duration,
}

impl ProvideDefaults for Config {
    fn defaults(_service_name: &str, _service_default_port: u16) -> impl Provider {
        Figment::from(("fast_handshake", !cfg!(debug_assertions)))
            .join(("skip_source_registration", false))
            .join((
                "youtube.public_url",
                Url::parse("https://www.youtube.com/watch/").unwrap(),
            ))
            .join(("youtube.max_upstream_requests", 1))
            // Max size of a youtbe page
            .join(("youtube.page_size", 50))
            .join(("youtube.expiration", "P1D"))
    }
}
