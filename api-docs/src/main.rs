use apelle_api_docs::MainError;
use apelle_common::{Reporter, service_main};

fn main() -> Result<(), Reporter<apelle_common::Error<MainError>>> {
    service_main("api-docs", 8079, apelle_api_docs::app)
}
