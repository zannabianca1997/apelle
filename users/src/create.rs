use std::collections::HashSet;

use apelle_common::db::{SqlError, SqlTx};
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
use snafu::Snafu;
use textwrap_macros::unfill;
use utoipa::{IntoResponses, openapi};

use crate::dtos::{UserCreateDto, UserDto};

#[derive(Debug, Snafu)]
pub enum CreateError {
    #[snafu(transparent)]
    SQLError {
        source: SqlError,
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

impl IntoResponses for CreateError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [
            (
                StatusCode::CONFLICT.as_str().to_owned(),
                openapi::RefOr::T(openapi::Response::new("User already exists")),
            ),
            (
                StatusCode::BAD_REQUEST.as_str().to_owned(),
                openapi::RefOr::T(openapi::Response::new("Invalid user name")),
            ),
        ]
        .into_iter()
        .chain(SqlError::responses())
        .collect()
    }
}

#[debug_handler(state=crate::App)]
#[utoipa::path(post, path = "/",
    responses((
        status = 201,
        description = "User created",
        content_type = "application/json",
        body = UserDto
    ), CreateError)
)]
/// Create a new user
///
/// Create a new user with the given name and password. No global role is
/// assigned. Name must be unique, not empty, not containing trailing or leading
/// spaces and control characters. It must also fit in a http header.
pub async fn create(
    mut tx: SqlTx,
    State(password_hasher): State<Argon2<'static>>,
    Json(UserCreateDto { name, password }): Json<UserCreateDto>,
) -> Result<(StatusCode, Json<UserDto>), CreateError> {
    // Check name validity
    if !check_name(&name) {
        return Err(CreateError::InvalidName { name });
    }

    // Hash the password
    let salt = SaltString::generate(&mut OsRng);
    let password = password_hasher
        .hash_password(password.as_bytes(), &salt)
        .unwrap();

    let Some((id, created, updated, last_login)) = sqlx::query_as(unfill!(
        "
        INSERT INTO apelle_user (name, password) VALUES ($1, $2)
        ON CONFLICT (name) DO NOTHING
        RETURNING id, created, updated, last_login
        "
    ))
    .bind(&name)
    .bind(password.to_string())
    .fetch_optional(&mut tx)
    .await
    .map_err(SqlError::from)?
    else {
        return Err(CreateError::Conflict { name });
    };

    Ok((
        StatusCode::CREATED,
        Json(UserDto {
            id,
            name,
            roles: HashSet::new(),
            created,
            updated,
            last_login,
        }),
    ))
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
