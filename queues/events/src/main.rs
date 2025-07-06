use apelle_common::{Reporter, service_main};
use apelle_queues_events::MainError;

fn main() -> Result<(), Reporter<apelle_common::Error<MainError>>> {
    service_main(
        "queues-events",
        env!("CARGO_PKG_VERSION"),
        8083,
        apelle_queues_events::app,
    )
}
