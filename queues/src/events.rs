use std::sync::Arc;

use apelle_common::{ServicesClient, common_errors::PubSubError, db::SqlTx};
use apelle_configs_dtos::QueueConfig;
use apelle_queues_events::events::{Event, Publisher};
use axum::{
    Extension, Json, debug_handler,
    extract::{Path, Query, State},
    response::{IntoResponse, NoContent},
};
use snafu::Snafu;
use utoipa::IntoResponses;

use crate::{
    QueuePathParams, Services,
    get::{GetError, GetQueryParams},
    middleware::user::QueueUser,
};

#[derive(Debug, Snafu)]
pub enum PushSyncEventError {
    #[snafu(transparent)]
    GetError { source: GetError },
    #[snafu(transparent)]
    PublishError { source: PubSubError },
}

impl IntoResponse for PushSyncEventError {
    fn into_response(self) -> axum::response::Response {
        match self {
            PushSyncEventError::GetError { source } => source.into_response(),
            PushSyncEventError::PublishError { source } => source.into_response(),
        }
    }
}

impl IntoResponses for PushSyncEventError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        GetError::responses()
            .into_iter()
            .chain(PubSubError::responses())
            .collect()
    }
}

#[debug_handler(state=crate::App)]
#[utoipa::path(post, path = "/push_sync_event", responses(PushSyncEventError))]
/// Force a sync event to be sent to the event stream
///
/// This will sent the entire state of the queue to the event stream,
/// ensuring the user has the correct state of the queue
pub async fn push_sync_event(
    tx: SqlTx,
    State(mut publisher): State<Publisher>,
    client: ServicesClient,
    State(services): State<Arc<Services>>,
    Extension(user): Extension<Arc<QueueUser>>,
    Extension(config): Extension<Arc<QueueConfig>>,
    Path(QueuePathParams { id }): Path<QueuePathParams>,
) -> Result<NoContent, PushSyncEventError> {
    let user_id = user.id();
    tracing::info!(queue=%id, user=%user_id, "Pushing sync event");

    let Json(state) = crate::get::get(
        tx,
        client,
        State(services),
        Extension(user),
        Extension(config),
        Query(GetQueryParams {
            config: false,
            songs: false,
            songs_source: false,
        }),
        Path(QueuePathParams { id }),
    )
    .await?;

    Event::user(id, user_id)
        .sync(state)
        .publish(&mut publisher)
        .await?;

    Ok(NoContent)
}
