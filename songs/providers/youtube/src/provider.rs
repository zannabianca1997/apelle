use std::{collections::HashMap, num::TryFromIntError, sync::Arc};

use apelle_common::{
    Reporter, TracingClient,
    common_errors::{SQLError, SQLSnafu},
};
use apelle_songs_dtos::provider::{ResolveQueryParams, ResolveResponse, SongsPathParams};
use axum::{
    Json, Router, debug_handler,
    extract::{Path, Query, State},
    http::HeaderName,
    response::{IntoResponse, NoContent},
    routing::{get, post, put},
};
use chrono::{DateTime, FixedOffset, Local};
use dtos::{
    PublicSongData, ResolveRequest,
    youtube::{ContentDetails, Paginated, Snippet, Thumbnail, Video},
};
use futures::{FutureExt, TryFutureExt, future::OptionFuture};
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use snafu::{ResultExt as _, Snafu};
use sqlx::{Executor as _, PgPool, QueryBuilder, Row as _, query_scalar};
use textwrap_macros::unfill;
use url::Url;

use crate::{App, YoutubeApi};
mod dtos;

#[debug_handler(state=crate::App)]
async fn ping() -> NoContent {
    tracing::debug!("Pinged on the provider interface");
    NoContent
}

#[derive(Debug, Serialize)]
struct YoutubeQueryParams<'a> {
    id: &'a str,
    part: &'a str,
}

const GOOGLE_API_KEY_HEADER: HeaderName = HeaderName::from_static("x-goog-api-key");

#[derive(Debug, Snafu)]
enum ResolveError {
    #[snafu(transparent)]
    SQLError {
        source: SQLError,
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

#[derive(Debug, Serialize, Deserialize)]
struct YoutubeSongData {
    video_id: String,
    etag: Option<String>,
    fetched: DateTime<FixedOffset>,
    thumbs: HashMap<String, Thumbnail>,
}

#[debug_handler(state=crate::App)]
async fn resolve(
    State(db): State<PgPool>,
    State(youtube_api): State<Arc<YoutubeApi>>,
    client: TracingClient,
    Query(ResolveQueryParams { public }): Query<ResolveQueryParams>,
    Json(ResolveRequest { video_id }): Json<ResolveRequest>,
) -> Result<Json<ResolveResponse<PublicSongData, YoutubeSongData>>, ResolveError> {
    // Checking if the song is already registered
    let know_id = query_scalar("SELECT id FROM youtube_song WHERE video_id = $1")
        .bind(&video_id)
        .fetch_optional(&db)
        .await
        .context(SQLSnafu)?;

    if let Some(id) = know_id {
        tracing::info!(%id, video_id, "Song already registered");

        let public = OptionFuture::from(public.then(|| async {
            let thumbs =
                sqlx::query_as("SELECT height, width, url FROM youtube_thumbs WHERE song_id = $1")
                    .bind(id)
                    .fetch_all(&db)
                    .await
                    .context(SQLSnafu)?
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

fn video_url(public_url: &Url, video_id: &str) -> Url {
    let mut url = public_url.clone();
    url.query_pairs_mut().append_pair("v", video_id);
    url
}

#[derive(Debug, Snafu)]
enum InsertError {
    #[snafu(transparent)]
    SQLError {
        source: SQLError,
    },
    Conflict,
}

impl IntoResponse for InsertError {
    fn into_response(self) -> axum::response::Response {
        match self {
            InsertError::SQLError { source } => source.into_response(),
            InsertError::Conflict => StatusCode::CONFLICT.into_response(),
        }
    }
}

#[debug_handler(state=crate::App)]
async fn put_songs(
    State(db): State<PgPool>,
    Path(SongsPathParams { id }): Path<SongsPathParams>,
    Json(YoutubeSongData {
        video_id,
        etag,
        fetched,
        thumbs,
    }): Json<YoutubeSongData>,
) -> Result<(StatusCode, NoContent), InsertError> {
    tracing::info!(%id, video_id, "Inserting song data");

    let mut qb =
        QueryBuilder::new("INSERT INTO youtube_song (id, video_id, etag, fetched) VALUES (");

    qb.separated(", ")
        .push_bind(id)
        .push_bind(video_id)
        .push_bind(etag)
        .push_bind(fetched);

    // On conflict for id: update, being this a PUT
    // On conflict for video_id: ignore, will rollback and get an error
    // Return if it was an update or not
    qb.push(unfill!(
        "
        )
        ON CONFLICT (id) DO UPDATE SET (video_id, etag, fetched) = (EXCLUDED.video_id, EXCLUDED.etag, EXCLUDED.fetched)
        RETURNING (NOT xmax = 0)
        "
    ));

    let song_insert_sql = qb.build();

    let mut qb =
        QueryBuilder::new("INSERT INTO youtube_thumbnail (song_id, height, width, url, size) ");

    qb.push_values(
        thumbs.into_iter(),
        |mut b, (size, Thumbnail { width, height, url })| {
            b.push_bind(id)
                .push_bind(height as i64)
                .push_bind(width as i64)
                .push_bind(url.to_string())
                .push_bind(size);
        },
    );

    qb.push(
        " ON CONFLICT (song_id, size)  DO UPDATE SET (url, width, height) = (EXCLUDED.url, EXCLUDED.width, EXCLUDED.height)"
    );

    let thumbs_insert_sql = qb.build();

    let mut tr = db.begin().await.context(SQLSnafu)?;

    let found = match tr.fetch_one(song_insert_sql).await {
        Ok(r) => r.get::<bool, _>(0),
        Err(sqlx::Error::Database(e)) if e.constraint() == Some("youtube_song_video_id") => {
            // This will also rollback the transaction :)
            return Err(InsertError::Conflict);
        }
        Err(source) => return Err(SQLError { source }.into()),
    };

    tr.execute(thumbs_insert_sql)
        .map(|r| r.context(SQLSnafu))
        .await?;

    tr.commit().await.context(SQLSnafu)?;

    Ok((
        if found {
            StatusCode::NO_CONTENT
        } else {
            StatusCode::CREATED
        },
        NoContent,
    ))
}

#[derive(Debug, Snafu)]
enum GetError {
    #[snafu(transparent)]
    SQLError {
        source: SQLError,
    },
    GetNotFound,
    GetDBInvalidThumbUrl {
        source: url::ParseError,
    },
    GetDBInvalidThumbSize {
        source: TryFromIntError,
    },
}

impl IntoResponse for GetError {
    fn into_response(self) -> axum::response::Response {
        match self {
            GetError::SQLError { source } => source.into_response(),
            GetError::GetNotFound => StatusCode::NOT_FOUND.into_response(),
            GetError::GetDBInvalidThumbUrl { source } => {
                tracing::error!("Invalid thumb url in the DB: {}", Reporter(source));
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            GetError::GetDBInvalidThumbSize { source } => {
                tracing::error!("Invalid thumb size in the DB: {}", Reporter(source));
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

#[debug_handler(state=crate::App)]
async fn get_songs(
    State(db): State<PgPool>,
    State(youtube_api): State<Arc<YoutubeApi>>,
    Path(SongsPathParams { id }): Path<SongsPathParams>,
) -> Result<Json<PublicSongData>, GetError> {
    tracing::info!(%id, "Retrieving song data");

    let video_id = sqlx::query_scalar("SELECT video_id FROM youtube_song WHERE id = $1")
        .bind(id)
        .fetch_optional(&db)
        .map(|r| r.context(SQLSnafu)?.ok_or(GetError::GetNotFound));

    let thumbs = async {
        sqlx::query_as("SELECT height, width, url FROM youtube_thumbnail WHERE song_id = $1")
            .bind(id)
            .fetch_all(&db)
            .await
            .context(SQLSnafu)?
            .into_iter()
            .map(|(height, width, url): (i32, i32, String)| {
                Ok::<_, GetError>(dtos::Thumbnail {
                    height: height.try_into().context(GetDBInvalidThumbSizeSnafu)?,
                    width: width.try_into().context(GetDBInvalidThumbSizeSnafu)?,
                    url: Url::parse(&url).context(GetDBInvalidThumbUrlSnafu)?,
                })
            })
            .collect::<Result<_, _>>()
    };

    let (video_id, thumbs): (String, _) = tokio::try_join!(video_id, thumbs)?;

    Ok(Json(PublicSongData {
        url: video_url(&youtube_api.public_url, &video_id),
        video_id,
        thumbs,
    }))
}

#[derive(Debug, Snafu)]
enum DeleteError {
    #[snafu(transparent)]
    SQLError {
        source: SQLError,
    },
    DeletedNotFound,
}

impl IntoResponse for DeleteError {
    fn into_response(self) -> axum::response::Response {
        match self {
            DeleteError::SQLError { source } => source.into_response(),
            DeleteError::DeletedNotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

#[debug_handler(state=crate::App)]
async fn delete_songs(
    State(db): State<PgPool>,
    Path(SongsPathParams { id }): Path<SongsPathParams>,
) -> Result<NoContent, DeleteError> {
    tracing::info!(%id, "Deleting song");

    let rows = sqlx::query("DELETE FROM youtube_song WHERE id = $1")
        .bind(id)
        .execute(&db)
        .await
        .context(SQLSnafu)?
        .rows_affected();

    if rows == 0 {
        return Err(DeleteError::DeletedNotFound);
    }

    Ok(NoContent)
}

pub fn provider() -> Router<App> {
    Router::new()
        .route("/", get(ping))
        .route("/resolve", post(resolve))
        .route(
            "/solved/{id}",
            put(put_songs).get(get_songs).delete(delete_songs),
        )
}
