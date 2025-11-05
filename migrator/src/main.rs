use apelle_common::{Reporter, helper_main};
use apelle_migrator::MainError;

fn main() -> Result<(), Reporter<apelle_common::HelperError<MainError>>> {
    helper_main("migrator", env!("CARGO_PKG_VERSION"), apelle_migrator::app)
}
