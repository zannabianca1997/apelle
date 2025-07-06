use apelle_common::{Reporter, service_main};
use apelle_users::MainError;

fn main() -> Result<(), Reporter<apelle_common::Error<MainError>>> {
    service_main("users", env!("CARGO_PKG_VERSION"), 8081, apelle_users::app)
}
