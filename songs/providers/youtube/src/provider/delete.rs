use super::SongsPathParams;
use axum::{debug_handler, extract::Path, response::NoContent};

#[debug_handler(state=crate::App)]
pub async fn delete(Path(SongsPathParams { id }): Path<SongsPathParams>) -> NoContent {
    tracing::info!(%id, "Received delete request");

    // Song was already deleted thanks to the ON DELETE CASCADE constraints

    NoContent
}
