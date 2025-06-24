use std::collections::HashSet;

use apelle_common::{
    AuthHeaders,
    common_errors::{SQLError, SQLSnafu},
};
use argon2::{
    Argon2, PasswordHasher as _,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Json, debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, NoContent},
};
use futures::FutureExt;
use snafu::{OptionExt, ResultExt, Snafu};
use sqlx::{PgPool, Row as _};
use textwrap_macros::unfill;
use utoipa::IntoResponses;
use uuid::Uuid;

use crate::{
    create::{self, check_name},
    dtos::{UserDto, UserUpdateDto},
};

#[debug_handler(state=crate::App)]
#[utoipa::path(get, path = "/me", responses((
    status=StatusCode::OK,
    description="Data about the user",
    body=UserDto
),SQLError))]
/// Current user data
///
/// Get data about the user the credentials used refer to.
pub async fn get(State(db): State<PgPool>, auth: AuthHeaders) -> Result<Json<UserDto>, SQLError> {
    let (created, updated, last_login) =
        sqlx::query_as("SELECT created, updated, last_login FROM apelle_user WHERE id = $1")
            .bind(auth.id())
            .fetch_one(&db)
            .await
            .context(SQLSnafu)?;

    Ok(Json(UserDto {
        id: auth.id(),
        name: auth.name().to_string(),
        roles: auth.roles().map(ToOwned::to_owned).collect(),
        created,
        updated,
        last_login,
    }))
}

#[derive(Debug, Snafu)]
pub enum UpdateError {
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

impl IntoResponse for UpdateError {
    fn into_response(self) -> axum::response::Response {
        match self {
            UpdateError::Conflict { .. } => StatusCode::CONFLICT.into_response(),
            UpdateError::SQLError { source } => source.into_response(),
            UpdateError::InvalidName { .. } => StatusCode::BAD_REQUEST.into_response(),
        }
    }
}

impl IntoResponses for UpdateError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        crate::create::CreateError::responses()
    }
}

#[debug_handler(state=crate::App)]
#[utoipa::path(patch, path = "/me", responses((
    status=StatusCode::OK,
    description="Patched data about the user",
    body=UserDto
),UpdateError))]
/// Patch current user data
///
/// Modify the datas about the user the credentials used refer to.
///
/// See the creation endpoint for the constraints.
pub async fn patch(
    State(db): State<PgPool>,
    State(password_hasher): State<Argon2<'static>>,
    auth: AuthHeaders,
    Json(UserUpdateDto { name, password }): Json<UserUpdateDto>,
) -> Result<Json<UserDto>, UpdateError> {
    if name.as_deref().is_some_and(|n| !check_name(n)) {
        return Err(UpdateError::InvalidName {
            name: name.unwrap(),
        });
    }

    let mut qb = sqlx::QueryBuilder::new("UPDATE apelle_user SET updated = NOW()");

    if let Some(name) = &name {
        qb.push(", name = ").push_bind(name);
    }

    if let Some(password) = password {
        // Hash the password
        let salt = SaltString::generate(&mut OsRng);
        let password = password_hasher
            .hash_password(password.as_bytes(), &salt)
            .unwrap();

        qb.push(", password = ").push_bind(password.to_string());
    }

    qb.push(" WHERE id = ").push_bind(auth.id());

    if let Some(name) = &name {
        qb.push("AND NOT EXISTS (SELECT 1 FROM apelle_user WHERE name = ")
            .push_bind(name)
            .push(" AND id != ")
            .push_bind(auth.id())
            .push(")");
    }

    let query = qb
        .push(" RETURNING  created, updated, last_login")
        .build_query_as();

    let (created, updated, last_login) = if let Some(name) = &name {
        query
            .fetch_optional(&db)
            .await
            .context(SQLSnafu)?
            .context(ConflictSnafu { name })?
    } else {
        query.fetch_one(&db).await.context(SQLSnafu)?
    };

    Ok(Json(UserDto {
        id: auth.id(),
        name: name.unwrap_or_else(|| auth.name().to_string()),
        roles: auth.roles().map(ToOwned::to_owned).collect(),
        created,
        updated,
        last_login,
    }))
}

#[debug_handler(state=crate::App)]
#[utoipa::path(delete, path = "/me", responses((
    status=StatusCode::NO_CONTENT,
    description="User deleted"
),SQLError))]
/// Delete current user
///
/// Delete the user the credentials used refer to.
pub async fn delete(State(db): State<PgPool>, auth: AuthHeaders) -> Result<NoContent, SQLError> {
    sqlx::query("DELETE FROM apelle_user WHERE id = $1")
        .bind(auth.id())
        .execute(&db)
        .await
        .context(SQLSnafu)?;
    Ok(NoContent)
}
