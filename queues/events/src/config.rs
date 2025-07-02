use std::collections::BTreeMap;

use apelle_common::{Figment, ProvideDefaults, Provider};
use serde::Deserialize;
use url::Url;

use crate::QueuesService;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Cache connection string
    pub pubsub_url: Url,

    /// Configuration for the queues service
    pub queues: QueuesService,

    /// Capacity of the inner event queue
    pub inner_queue_capacity: usize,
}

impl ProvideDefaults for Config {
    fn defaults(_service_name: &str, _service_default_port: u16) -> impl Provider {
        Figment::new()
            .join(("queues", BTreeMap::from([("sync_timeout", "PT2S")])))
            .join(("inner_queue_capacity", 1024))
    }
}
