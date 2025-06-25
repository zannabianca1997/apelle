use std::{num::TryFromIntError, sync::Arc};

use apelle_common::{
    Reporter, ServicesClient,
    db::{SqlError, SqlTx},
};
use apelle_songs_dtos::provider::{ResolveQueryParams, ResolveResponse};
use axum::{
    Json, debug_handler,
    extract::{Query, State},
    response::IntoResponse,
};
use chrono::Local;
use futures::{FutureExt, TryFutureExt, future::OptionFuture};
use reqwest::{Response, StatusCode};
use serde::Serialize;
use snafu::{ResultExt as _, Snafu};
use sqlx::query_scalar;
use url::Url;

use super::dtos::{
    self, PublicSongData, ResolveRequest,
    youtube::{ContentDetails, Paginated, Snippet, Video},
};
use super::{GOOGLE_API_KEY_HEADER, YoutubeSongData, video_url};
use crate::YoutubeApi;

#[derive(Debug, Serialize)]
struct YoutubeQueryParams<'a> {
    id: &'a str,
    part: &'a str,
}

#[derive(Debug, Snafu)]
pub enum ResolveError {
    #[snafu(transparent)]
    SQLError {
        source: SqlError,
    },
    #[snafu(transparent)]
    RequestError {
        source: reqwest::Error,
    },
    MultipleResults {
        count: usize,
    },
    NotFound,
    DBInvalidThumbUrl {
        source: url::ParseError,
    },

    DBInvalidThumbSize {
        source: TryFromIntError,
    },
}

impl IntoResponse for ResolveError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ResolveError::RequestError { source } => {
                tracing::error!("Request to the youtube api error: {}", Reporter(source));
                StatusCode::BAD_GATEWAY.into_response()
            }
            ResolveError::MultipleResults { count } => {
                tracing::error!(
                    count,
                    "Request to the youtube api returned multiple results"
                );
                StatusCode::BAD_REQUEST.into_response()
            }
            ResolveError::NotFound => StatusCode::NOT_FOUND.into_response(),
            ResolveError::SQLError { source } => source.into_response(),
            ResolveError::DBInvalidThumbUrl { source } => {
                tracing::error!("Invalid thumb url in the DB: {}", Reporter(source));
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            ResolveError::DBInvalidThumbSize { source } => {
                tracing::error!("Invalid thumb size in the DB: {}", Reporter(source));
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

#[debug_handler(state=crate::App)]
pub async fn resolve(
    mut tx: SqlTx,
    State(youtube_api): State<Arc<YoutubeApi>>,
    client: ServicesClient,
    Query(ResolveQueryParams { public }): Query<ResolveQueryParams>,
    Json(ResolveRequest { video_id }): Json<ResolveRequest>,
) -> Result<Json<ResolveResponse<PublicSongData, YoutubeSongData>>, ResolveError> {
    // Checking if the song is already registered
    let know_id = query_scalar("SELECT id FROM youtube_song WHERE video_id = $1")
        .bind(&video_id)
        .fetch_optional(&mut tx)
        .await
        .map_err(SqlError::from)?;

    if let Some(id) = know_id {
        tracing::info!(%id, video_id, "Song already registered");

        let public = OptionFuture::from(public.then_some(async {
            let thumbs = sqlx::query_as(
                "SELECT height, width, url FROM youtube_thumbnail WHERE song_id = $1",
            )
            .bind(id)
            .fetch_all(&mut tx)
            .await
            .map_err(SqlError::from)?
            .into_iter()
            .map(|(height, width, url): (i32, i32, String)| {
                Ok::<_, ResolveError>(dtos::Thumbnail {
                    height: height.try_into().context(DBInvalidThumbSizeSnafu)?,
                    width: width.try_into().context(DBInvalidThumbSizeSnafu)?,
                    url: Url::parse(&url).context(DBInvalidThumbUrlSnafu)?,
                })
            })
            .collect::<Result<_, _>>()?;

            Ok::<_, ResolveError>(PublicSongData {
                url: video_url(&youtube_api.public_url, &video_id),
                video_id,
                thumbs,
            })
        }))
        .await
        .transpose()?;

        return Ok(Json(ResolveResponse::Existing { id, public }));
    }

    let YoutubeApi {
        api_key,
        api_list_url: api_url,
        public_url,
        ..
    } = &*youtube_api;
    let fetched = Local::now().into();

    tracing::info!(video_id, "Retrieving song data");
    let Paginated { items, .. } = client
        .get(api_url.clone())
        .query(&YoutubeQueryParams {
            id: &video_id,
            part: "snippet,contentDetails",
        })
        .header(GOOGLE_API_KEY_HEADER, api_key)
        .send()
        .map(|r| r.and_then(Response::error_for_status))
        .and_then(Response::json)
        .await?;
    if items.len() > 1 {
        return Err(ResolveError::MultipleResults { count: items.len() });
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
        return Err(ResolveError::NotFound);
    };

    Ok(Json(ResolveResponse::New {
        title,
        duration,
        public: public.then(|| PublicSongData {
            video_id: video_id.clone(),
            url: video_url(public_url, &video_id),
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
