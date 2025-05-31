use std::collections::HashSet;

use apelle_common::Reporter;
use argon2::{
    Argon2, PasswordHasher as _,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, debug_handler, extract::State, http, response::IntoResponse};
use snafu::{ResultExt, Snafu};
use sqlx::PgPool;

use crate::dtos::{UserCreateDto, UserDto};

#[derive(Debug, Snafu)]
pub enum CreateError {
    SQLError { source: sqlx::Error },
    Conflict { name: String },
}

impl IntoResponse for CreateError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CreateError::Conflict { .. } => http::StatusCode::CONFLICT.into_response(),
            CreateError::SQLError { source } => {
                tracing::error!("SQL error: {}", Reporter(source));
                http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

#[debug_handler(state=crate::App)]
pub async fn create(
    State(db): State<PgPool>,
    State(password_hasher): State<Argon2<'static>>,
    Json(UserCreateDto { name, password }): Json<UserCreateDto>,
) -> Result<Json<UserDto>, CreateError> {
    // Hash the password
    let salt = SaltString::generate(&mut OsRng);
    let password = password_hasher
        .hash_password(password.as_bytes(), &salt)
        .unwrap();

    let Some((id, created, updated, last_login)) = sqlx::query_as(
        "
            INSERT INTO apelle_user (name, password)
            VALUES ($1, $2)
            ON CONFLICT (name) DO NOTHING
            RETURNING id, created, updated, last_login
        ",
    )
    .bind(&name)
    .bind(password.to_string())
    .fetch_optional(&db)
    .await
    .context(SQLSnafu)?
    else {
        return Err(CreateError::Conflict { name });
    };

    Ok(Json(UserDto {
        id,
        name,
        roles: HashSet::new(),
        created,
        updated,
        last_login,
    }))
}
