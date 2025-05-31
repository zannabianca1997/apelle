use std::collections::HashSet;

use apelle_common::common_errors::{SQLError, SQLSnafu};
use argon2::{
    Argon2, PasswordHasher as _,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Json, debug_handler,
    extract::State,
    http::{HeaderValue, StatusCode},
    response::IntoResponse,
};
use snafu::{ResultExt, Snafu};
use sqlx::PgPool;

use crate::dtos::{UserCreateDto, UserDto};

#[derive(Debug, Snafu)]
pub enum CreateError {
    #[snafu(transparent)]
    SQLError {
        source: SQLError,
    },
    Conflict {
        name: String,
    },
    InvalidName {
        name: String,
    },
}

impl IntoResponse for CreateError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CreateError::Conflict { .. } => StatusCode::CONFLICT.into_response(),
            CreateError::SQLError { source } => source.into_response(),
            CreateError::InvalidName { .. } => StatusCode::BAD_REQUEST.into_response(),
        }
    }
}

#[debug_handler(state=crate::App)]
pub async fn create(
    State(db): State<PgPool>,
    State(password_hasher): State<Argon2<'static>>,
    Json(UserCreateDto { name, password }): Json<UserCreateDto>,
) -> Result<Json<UserDto>, CreateError> {
    // Check name validity
    if !check_name(&name) {
        return Err(CreateError::InvalidName { name });
    }

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

pub fn check_name(name: &str) -> bool {
    // No whitespace around
    name.trim() == name
        // Not empty
        && !name.is_empty()
        // No control characters (checks also for newlines)
        && !name.chars().any(char::is_control)
        // Should fit in a header
        && HeaderValue::from_bytes(name.as_bytes()).is_ok()
}
