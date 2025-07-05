//! Middleware that add the `player_state_id` as ETag on all responses, and
//! refuses the ones that don't match.

use std::{convert::Infallible, str::FromStr, time::SystemTime};

use apelle_common::{
    ResponseOrNotModified,
    db::{SqlError, SqlTx},
};
use apelle_queues_events::events::{BuildPatchEvent, Collector, Event};
use axum::{
    Extension, debug_middleware,
    extract::{Path, Request},
    http::Method,
    middleware::Next,
    response::{IntoResponse, IntoResponseParts, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{ETag, IfMatch, IfModifiedSince, IfNoneMatch, IfUnmodifiedSince, LastModified},
};
use chrono::{DateTime, FixedOffset};
use reqwest::StatusCode;
use snafu::Snafu;
use textwrap_macros::unfill;
use uuid::Uuid;

use crate::QueuePathParams;

/// Extension signaling that the queue has changed and so a new ETag should be
/// generated
#[derive(Debug, Clone, Copy)]
struct ETagInfo {
    player_state_id: Uuid,
    updated: DateTime<FixedOffset>,
}

#[derive(Debug, Snafu)]
pub enum EtagError {
    #[snafu(transparent)]
    Sql {
        source: SqlError,
    },
    NotFound,

    PreconditionFailed,
}

impl IntoResponse for EtagError {
    fn into_response(self) -> Response {
        match self {
            EtagError::Sql { source } => source.into_response(),
            EtagError::NotFound => StatusCode::NOT_FOUND.into_response(),
            EtagError::PreconditionFailed => StatusCode::PRECONDITION_FAILED.into_response(),
        }
    }
}

#[debug_middleware(state=crate::App)]
pub async fn etag_middleware(
    mut tx: SqlTx,
    if_match: Option<TypedHeader<IfMatch>>,
    if_none_match: Option<TypedHeader<IfNoneMatch>>,
    if_modified_since: Option<TypedHeader<IfModifiedSince>>,
    if_unmodified_since: Option<TypedHeader<IfUnmodifiedSince>>,
    method: Method,
    Path(QueuePathParams { id }): Path<QueuePathParams>,
    request: Request,
    next: Next,
) -> Result<
    (
        TypedHeader<ETag>,
        TypedHeader<LastModified>,
        ResponseOrNotModified<Response>,
    ),
    EtagError,
> {
    let (last_modified, etag) =
        sqlx::query_as("SELECT updated, player_state_id FROM queue WHERE id = $1")
            .bind(id)
            .fetch_optional(&mut tx)
            .await
            .map_err(SqlError::from)?
            .map(|(updated, r): (DateTime<FixedOffset>, Uuid)| {
                (
                    LastModified::from(SystemTime::from(updated)),
                    ETag::from_str(&format!("\"{r}\"")).unwrap(),
                )
            })
            .ok_or(EtagError::NotFound)?;

    if let Method::GET | Method::HEAD = method {
        // Read method: check the queue has changed

        if if_none_match.is_some_and(|if_none_match| !if_none_match.precondition_passes(&etag))
            || if_modified_since.is_some_and(|if_modified_since| {
                !if_modified_since.is_modified(last_modified.into())
            })
        {
            // The queue was not acted upon from the tags provided, so we
            // can avoid returning it
            return Ok((
                TypedHeader(etag),
                TypedHeader(last_modified),
                ResponseOrNotModified::NotModified,
            ));
        }
    } else {
        // Write method: check the queue hasn't changed

        if if_match.is_some_and(|if_match| !if_match.precondition_passes(&etag))
            || if_unmodified_since.is_some_and(|if_unmodified_since| {
                !if_unmodified_since.precondition_passes(last_modified.into())
            })
        {
            // The queue changed from the state provided, so the request
            // cannot go through
            return Err(EtagError::PreconditionFailed);
        }
    }

    // Dropping the handle to the transaction, as the rest of the chain will need it
    drop(tx);

    let mut response = next.run(request).await;

    let (etag, last_modified) = response
        .extensions_mut()
        .remove()
        .map(
            |ETagInfo {
                 player_state_id,
                 updated,
             }| {
                (
                    ETag::from_str(&format!("\"{player_state_id}\"")).unwrap(),
                    LastModified::from(SystemTime::from(updated)),
                )
            },
        )
        .unwrap_or((etag, last_modified));

    Ok((
        TypedHeader(etag),
        TypedHeader(last_modified),
        ResponseOrNotModified::Response(response),
    ))
}

pub struct Changed {
    info: ETagInfo,
}

impl Changed {
    pub async fn new(
        tx: &mut SqlTx,
        collector: &Collector<5>,
        queue_id: Uuid,
    ) -> Result<Self, SqlError> {
        let (player_state_id, updated) = sqlx::query_as(unfill!(
            "
            UPDATE queue 
            SET 
            player_state_id = gen_random_uuid(), 
            updated = NOW() 
            RETURNING player_state_id, updated 
            WHERE id = $1
            "
        ))
        .bind(queue_id)
        .fetch_one(tx)
        .await?;

        Event::queue(queue_id)
            .replace("/player_state_id", player_state_id)
            .replace("/updated", updated)
            .build()
            .collect(collector)
            .await;

        Ok(Self {
            info: ETagInfo {
                player_state_id,
                updated,
            },
        })
    }
}

impl IntoResponseParts for Changed {
    type Error = Infallible;
    fn into_response_parts(
        self,
        res: axum::response::ResponseParts,
    ) -> Result<axum::response::ResponseParts, Self::Error> {
        Extension(self.info).into_response_parts(res)
    }
}
