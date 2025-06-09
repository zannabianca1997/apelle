use apelle_common::{service_main, Reporter};
use apelle_queues::MainError;

fn main() -> Result<(), Reporter<apelle_common::Error<MainError>>> {
    service_main("queues", 8083, apelle_queues::app)
}
