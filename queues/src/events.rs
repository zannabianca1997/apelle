use std::sync::Arc;

use crate::{QueuePathParams, middleware::user::QueueUser};
use apelle_configs_dtos::{QueueUserAction, QueueUserActionQueue};
use axum::{
    Extension, debug_handler,
    extract::Path,
    response::{IntoResponse, Redirect},
};
use reqwest::StatusCode;
use snafu::Snafu;
use utoipa::{IntoResponses, openapi};

#[derive(Debug, Clone, Copy, Snafu)]
pub struct Forbidden;

impl IntoResponse for Forbidden {
    fn into_response(self) -> axum::response::Response {
        StatusCode::FORBIDDEN.into_response()
    }
}

impl IntoResponses for Forbidden {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [(
            StatusCode::FORBIDDEN.as_str().to_string(),
            openapi::Response::new("User is not allowed to read the queue events").into(),
        )]
        .into_iter()
        .collect()
    }
}

/// Read the queue data
#[debug_handler(state = crate::App)]
#[utoipa::path(get, path = "/events",
    responses(
        (status = StatusCode::OK, description = "Queue events", content_type = "text/event-stream"),
        Forbidden
    ),
    params(QueuePathParams)
)]
pub async fn events(
    Extension(user): Extension<Arc<QueueUser>>,
    Path(QueuePathParams { id }): Path<QueuePathParams>,
) -> Result<Redirect, Forbidden> {
    if !user.can(QueueUserAction::Queue(QueueUserActionQueue::Get)) {
        return Err(Forbidden);
    }

    Ok(Redirect::temporary(&format!("/events/{id}")))
}
