use apelle_common::{Reporter, service_main};
use apelle_queues::MainError;

fn main() -> Result<(), Reporter<apelle_common::ServiceError<MainError>>> {
    service_main(
        "queues",
        env!("CARGO_PKG_VERSION"),
        8083,
        apelle_queues::app,
    )
}
