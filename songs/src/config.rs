use apelle_common::{Figment, ProvideDefaults, Provider};
use chrono::Duration;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Database connection string
    pub db_url: Url,
    /// Cache connection string
    pub cache_url: Url,

    /// If requests to skip webhook checking should be honored
    pub honor_fast_handshake: bool,

    pub seen_sources_queue_size: usize,

    #[serde(with = "apelle_common::iso8601::duration")]
    pub cache_expiration: Duration,
}

impl ProvideDefaults for Config {
    fn defaults(_service_name: &str, _service_default_port: u16) -> impl Provider {
        Figment::new()
            .join(("honor_fast_handshake", true))
            .join(("seen_sources_queue_size", 50))
            .join(("cache_expiration", "P1D"))
            .join(("page_size", 10))
    }
}
