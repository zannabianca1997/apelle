use std::{num::TryFromIntError, sync::Arc};

use apelle_common::{
    Reporter,
    db::{SqlError, SqlTx},
};
use apelle_songs_dtos::provider::SongsPathParams;
use axum::{
    Json, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use snafu::{OptionExt, ResultExt as _, Snafu};
use url::Url;

use super::dtos::{self, PublicSongData};
use super::video_url;
use crate::YoutubeApi;

#[derive(Debug, Snafu)]
pub enum GetError {
    #[snafu(transparent)]
    SQLError {
        source: SqlError,
    },
    NotFound,
    DBInvalidThumbUrl {
        source: url::ParseError,
    },
    DBInvalidThumbSize {
        source: TryFromIntError,
    },
}

impl IntoResponse for GetError {
    fn into_response(self) -> axum::response::Response {
        match self {
            GetError::SQLError { source } => source.into_response(),
            GetError::NotFound => StatusCode::NOT_FOUND.into_response(),
            GetError::DBInvalidThumbUrl { source } => {
                tracing::error!("Invalid thumb url in the DB: {}", Reporter(source));
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            GetError::DBInvalidThumbSize { source } => {
                tracing::error!("Invalid thumb size in the DB: {}", Reporter(source));
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

#[debug_handler(state=crate::App)]
pub async fn get(
    mut tx: SqlTx,
    State(youtube_api): State<Arc<YoutubeApi>>,
    Path(SongsPathParams { id }): Path<SongsPathParams>,
) -> Result<Json<PublicSongData>, GetError> {
    tracing::info!(%id, "Retrieving song data");

    let video_id: String = sqlx::query_scalar("SELECT video_id FROM youtube_song WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut tx)
        .await
        .map_err(SqlError::from)?
        .context(NotFoundSnafu)?;

    let thumbs =
        sqlx::query_as("SELECT height, width, url FROM youtube_thumbnail WHERE song_id = $1")
            .bind(id)
            .fetch_all(&mut tx)
            .await
            .map_err(SqlError::from)?
            .into_iter()
            .map(|(height, width, url): (i32, i32, String)| {
                Ok::<_, GetError>(dtos::Thumbnail {
                    height: height.try_into().context(DBInvalidThumbSizeSnafu)?,
                    width: width.try_into().context(DBInvalidThumbSizeSnafu)?,
                    url: Url::parse(&url).context(DBInvalidThumbUrlSnafu)?,
                })
            })
            .collect::<Result<_, _>>()?;

    Ok(Json(PublicSongData {
        url: video_url(&youtube_api.public_url, &*video_id),
        video_id,
        thumbs,
    }))
}
