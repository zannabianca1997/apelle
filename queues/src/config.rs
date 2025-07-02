use apelle_common::{Figment, ProvideDefaults, Provider, Serialized};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Database connection string
    pub db_url: Url,

    /// Cache connection string
    pub cache_url: Url,

    /// Url of the `songs` service
    pub songs_url: Url,

    /// Url of the `configs` service
    pub configs_url: Url,

    /// Configuration for the code generator
    pub code: CodeConfig,

    /// Url of the `queues-events` service
    pub events_url: Url,
}

impl ProvideDefaults for Config {
    fn defaults(_service_name: &str, _service_default_port: u16) -> impl Provider {
        Figment::new().join(Serialized::default("code", CodeConfig::default()))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeConfig {
    /// Allowed characters in the generated codes
    pub alphabet: String,

    /// Minimum complexity of the generated codes
    pub min_bits: u32,
    /// How much the complexity is increased at each retry
    pub retry_bits: u32,
}

impl Default for CodeConfig {
    fn default() -> Self {
        Self {
            alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string(),
            min_bits: 24, // About a 4 character code with the standard alphabet
            retry_bits: 1,
        }
    }
}
