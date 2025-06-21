use std::collections::HashMap;

use apelle_common::{Figment, ProvideDefaults, Provider};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Services to track
    pub services: HashMap<String, Service>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Service {
    /// Internal url of the service
    pub url: Url,
    /// Public endpoint of the service
    #[serde(default)]
    pub public: Option<String>,
}

impl ProvideDefaults for Config {
    fn defaults(_service_name: &str, _service_default_port: u16) -> impl Provider {
        Figment::new()
    }
}
