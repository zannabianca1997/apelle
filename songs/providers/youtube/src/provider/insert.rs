use apelle_common::common_errors::{SQLError, SQLSnafu};
use apelle_songs_dtos::provider::SongsPathParams;

use axum::{
    Json, debug_handler,
    extract::{Path, State},
    response::{IntoResponse, NoContent},
};
use futures::FutureExt;
use reqwest::StatusCode;
use snafu::{ResultExt as _, Snafu};
use sqlx::{Executor as _, PgPool, Row as _};
use textwrap_macros::unfill;

use crate::provider::dtos::youtube::Thumbnail;

use super::YoutubeSongData;

#[derive(Debug, Snafu)]
pub enum InsertError {
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
pub async fn insert(
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

    // On conflict for id: update, being this a PUT
    // On conflict for video_id: ignore, will rollback and get an error
    // Return if it was an update or not
    let song_insert_sql = sqlx::query(unfill!(
        "
        INSERT INTO youtube_song (id, video_id, etag, fetched) 
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (id) DO UPDATE SET (video_id, etag, fetched) = (EXCLUDED.video_id, EXCLUDED.etag, EXCLUDED.fetched)
        RETURNING (NOT xmax = 0)
        "
    )
    .trim_ascii())
    .bind(id)
    .bind(video_id)
    .bind(etag)
    .bind(fetched);

    let mut heights = Vec::with_capacity(thumbs.len());
    let mut widths = Vec::with_capacity(thumbs.len());
    let mut urls = Vec::with_capacity(thumbs.len());
    let mut sizes = Vec::with_capacity(thumbs.len());
    for (size, Thumbnail { width, height, url }) in thumbs {
        heights.push(height as i64);
        widths.push(width as i64);
        urls.push(url.to_string());
        sizes.push(size);
    }

    let thumbs_insert_sql = sqlx::query(
        unfill!(
            "
            INSERT INTO youtube_thumbnail (song_id, height, width, url, size)
            SELECT $1, * FROM UNNEST($2::int4[], $3::int4[], $4::text[], $5::text[])
            ON CONFLICT (song_id, size)  
            DO UPDATE SET (url, width, height) = (EXCLUDED.url, EXCLUDED.width, EXCLUDED.height)
            "
        )
        .trim_ascii(),
    )
    .bind(id)
    .bind(&heights)
    .bind(&widths)
    .bind(&urls)
    .bind(&sizes);

    let mut tr = db.begin().await.context(SQLSnafu)?;

    let found = match tr.fetch_one(song_insert_sql).await {
        Ok(r) => r.get::<bool, _>(0),
        Err(sqlx::Error::Database(e)) if e.constraint() == Some("youtube_song_video_id") => {
            tr.rollback().await.context(SQLSnafu)?;
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
