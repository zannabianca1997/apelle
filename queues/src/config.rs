use apelle_common::{Figment, ProvideDefaults, Provider};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Database connection string
    pub db_url: Url,

    /// Cache connection string
    pub cache_url: Url,

    /// Url of the `songs` service
    pub songs_url: Url,
}

impl ProvideDefaults for Config {
    fn defaults(_service_name: &str, _service_default_port: u16) -> impl Provider {
        Figment::new()
    }
}
