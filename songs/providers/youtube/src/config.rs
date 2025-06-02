use std::collections::HashMap;

use apelle_common::{ProvideDefaults, Provider, Serialized};
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
}

impl ProvideDefaults for Config {
    fn defaults(_service_name: &str, _service_default_port: u16) -> impl Provider {
        Serialized::defaults(HashMap::from([
            ("fast_handshake", !cfg!(debug_assertions)),
            ("skip_source_registration", false),
        ]))
    }
}
