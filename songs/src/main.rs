use apelle_common::{Reporter, service_main};
use apelle_songs::MainError;

fn main() -> Result<(), Reporter<apelle_common::Error<MainError>>> {
    service_main("songs", 8082, apelle_songs::app)
}
