use apelle_common::{Reporter, service_main};
use apelle_songs_youtube::MainError;

fn main() -> Result<(), Reporter<apelle_common::Error<MainError>>> {
    service_main(
        "songs-youtube",
        env!("CARGO_PKG_VERSION"),
        8082,
        apelle_songs_youtube::app,
    )
}
