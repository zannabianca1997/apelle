use apelle_common::{service_main, Reporter};
use apelle_configs::MainError;

fn main() -> Result<(), Reporter<apelle_common::Error<MainError>>> {
    service_main("configs", 8084, apelle_configs::app)
}
