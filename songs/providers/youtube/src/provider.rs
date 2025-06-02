use std::{collections::HashMap, sync::Arc};

use apelle_common::Reporter;
use apelle_songs_dtos::provider::{RetrieveQueryParams, RetrieveResponse};
use axum::{
    Json, Router, debug_handler,
    extract::{Query, State},
    response::{IntoResponse, NoContent},
    routing::{get, post},
};
use chrono::{DateTime, FixedOffset, Local};
use dtos::{
    PublicSongData, RetrieveRequest,
    youtube::{ContentDetails, Paginated, Snippet, Thumbnail, Video},
};
use futures::{FutureExt, TryFutureExt};
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

use crate::{App, YoutubeApi};
mod dtos;

#[debug_handler(state=crate::App)]
async fn ping() -> NoContent {
    tracing::debug!("Pinged on the provider interface");
    NoContent
}

#[derive(Debug, Serialize)]
struct YoutubeQueryParams<'a> {
    key: &'a str,
    id: &'a str,
    part: &'a str,
}

#[derive(Debug, Snafu)]
enum RetrieveError {
    #[snafu(transparent)]
    RequestError {
        source: reqwest::Error,
    },
    MultipleResults {
        count: usize,
    },
    NotFound,
}

impl IntoResponse for RetrieveError {
    fn into_response(self) -> axum::response::Response {
        match self {
            RetrieveError::RequestError { source } => {
                tracing::error!("Request to the youtube api error: {}", Reporter(source));
                StatusCode::BAD_GATEWAY.into_response()
            }
            RetrieveError::MultipleResults { count } => {
                tracing::error!(
                    count,
                    "Request to the youtube api returned multiple results"
                );
                StatusCode::BAD_REQUEST.into_response()
            }
            RetrieveError::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct YoutubeSongData {
    video_id: String,
    etag: Option<String>,
    fetched: DateTime<FixedOffset>,
    thumbs: HashMap<String, Thumbnail>,
}

#[debug_handler(state=crate::App)]
async fn retrieve(
    State(youtube_api): State<Arc<YoutubeApi>>,
    State(client): State<reqwest::Client>,
    Query(RetrieveQueryParams { public }): Query<RetrieveQueryParams>,
    Json(RetrieveRequest { video_id }): Json<RetrieveRequest>,
) -> Result<Json<RetrieveResponse<PublicSongData, YoutubeSongData>>, RetrieveError> {
    let YoutubeApi {
        api_key,
        api_list_url: api_url,
        public_url,
        ..
    } = &*youtube_api;
    let fetched = Local::now().into();

    let Paginated { items, .. } = client
        .get(api_url.clone())
        .query(&YoutubeQueryParams {
            key: api_key,
            id: &video_id,
            part: "snippet,contentDetails",
        })
        .send()
        .map(|r| r.and_then(Response::error_for_status))
        .and_then(Response::json)
        .await?;
    if items.len() > 1 {
        return Err(RetrieveError::MultipleResults { count: items.len() });
    }
    let Some(Video {
        etag,
        snippet: Snippet {
            title,
            thumbnails: thumbs,
        },
        content_details: ContentDetails { duration },
        ..
    }) = items.into_iter().next()
    else {
        return Err(RetrieveError::NotFound);
    };

    let mut url = public_url.clone();
    url.query_pairs_mut().append_pair("v", &video_id);

    Ok(Json(RetrieveResponse {
        title,
        duration,
        public: public.then(|| PublicSongData {
            video_id: video_id.clone(),
            url,
            thumbs: thumbs.values().cloned().map(Into::into).collect(),
        }),
        callback: Some(YoutubeSongData {
            video_id,
            etag,
            fetched,
            thumbs,
        }),
    }))
}

pub fn provider() -> Router<App> {
    Router::new()
        .route("/", get(ping))
        .route("/retrieve", post(retrieve))
}
