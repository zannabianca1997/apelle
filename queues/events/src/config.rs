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

    /// Configuration for the SSE stream
    pub sse: SseConfig,
}

impl ProvideDefaults for Config {
    fn defaults(_service_name: &str, _service_default_port: u16) -> impl Provider {
        Figment::new()
            .join(("queues", BTreeMap::from([("sync_timeout", "PT2S")])))
            .join(("inner_queue_capacity", 1024))
            .join(("sse", BTreeMap::from([("chunk_size", 10)])))
            .join((
                "sse",
                BTreeMap::from([("throttle", "PT0.1S"), ("keep_alive", "PT15S")]),
            ))
    }
}

#[derive(Debug, Clone, Deserialize, Copy)]
pub struct SseConfig {
    pub chunk_size: usize,
    #[serde(with = "apelle_common::iso8601::duration")]
    pub throttle: std::time::Duration,
    #[serde(with = "apelle_common::iso8601::opt_duration")]
    pub keep_alive: Option<std::time::Duration>,
}
