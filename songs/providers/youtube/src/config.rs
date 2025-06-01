use apelle_common::{Figment, ProvideDefaults, Provider};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Url this service is reachable from `songs`
    pub self_url: Url,

    /// Url of the `songs` service
    pub songs_url: Url,
}

impl ProvideDefaults for Config {
    fn defaults(_service_name: &str, _service_default_port: u16) -> impl Provider {
        Figment::new()
    }
}
