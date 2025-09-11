use apelle_common::db::{SqlError, SqlTx};
use apelle_songs_dtos::provider::SongsPathParams;

use axum::{
    Json, debug_handler,
    extract::Path,
    response::{IntoResponse, NoContent},
};
use reqwest::StatusCode;
use snafu::Snafu;

use crate::provider::dtos::youtube::Thumbnail;

use super::YoutubeSongData;

#[derive(Debug, Snafu)]
pub enum InsertError {
    #[snafu(transparent)]
    SQLError {
        source: SqlError,
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
    mut tx: SqlTx,
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
    let song_insert_sql = sqlx::query_scalar!(
        r#"
        INSERT INTO youtube_song (id, video_id, etag, fetched) 
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (id) DO UPDATE SET (video_id, etag, fetched) = (EXCLUDED.video_id, EXCLUDED.etag, EXCLUDED.fetched)
        RETURNING (NOT xmax = 0) AS "found!"
        "#,
        id,
        video_id,
        etag,
        fetched
    );

    let mut heights = Vec::with_capacity(thumbs.len());
    let mut widths = Vec::with_capacity(thumbs.len());
    let mut urls = Vec::with_capacity(thumbs.len());
    let mut sizes = Vec::with_capacity(thumbs.len());
    for (size, Thumbnail { width, height, url }) in thumbs {
        heights.push(height as i32);
        widths.push(width as i32);
        urls.push(url.to_string());
        sizes.push(size);
    }

    let thumbs_insert_sql = sqlx::query!(
        "
        INSERT INTO youtube_thumbnail (song_id, height, width, url, size)
        SELECT $1, * FROM UNNEST($2::int4[], $3::int4[], $4::text[], $5::text[])
        ON CONFLICT (song_id, size)  
        DO UPDATE SET (url, width, height) = (EXCLUDED.url, EXCLUDED.width, EXCLUDED.height)
        ",
        id,
        &heights,
        &widths,
        &urls,
        &sizes
    );

    let found = match song_insert_sql.fetch_one(&mut tx).await {
        Ok(r) => r,
        Err(sqlx::Error::Database(e)) if e.constraint() == Some("youtube_song_video_id") => {
            return Err(InsertError::Conflict);
        }
        Err(source) => {
            return Err(InsertError::SQLError {
                source: source.into(),
            });
        }
    };

    thumbs_insert_sql
        .execute(&mut tx)
        .await
        .map_err(SqlError::from)?;

    Ok((
        if found {
            StatusCode::NO_CONTENT
        } else {
            StatusCode::CREATED
        },
        NoContent,
    ))
}
