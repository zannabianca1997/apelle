use apelle_common::{error_reporter::Reporter, service_main};
use apelle_songs::MainError;

fn main() -> Result<(), Reporter<apelle_common::main_wrapper::Error<MainError>>> {
    service_main("songs", 8082, apelle_songs::app)
}
