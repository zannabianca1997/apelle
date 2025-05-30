use apelle_common::{error_reporter::Reporter, service_main};
use apelle_users::MainError;

fn main() -> Result<(), Reporter<apelle_common::main_wrapper::Error<MainError>>> {
    service_main("users", apelle_users::app)
}
