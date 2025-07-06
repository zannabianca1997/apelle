use apelle_common::{Reporter, service_main};
use apelle_configs::MainError;

fn main() -> Result<(), Reporter<apelle_common::Error<MainError>>> {
    service_main(
        "configs",
        env!("CARGO_PKG_VERSION"),
        8084,
        apelle_configs::app,
    )
}
