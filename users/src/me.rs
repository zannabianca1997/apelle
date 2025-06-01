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
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, NoContent},
};
use futures::FutureExt;
use snafu::{OptionExt, ResultExt, Snafu};
use sqlx::{PgPool, Row as _};
use uuid::Uuid;

use crate::{
    create::check_name,
    dtos::{UserDto, UserUpdateDto},
};

pub async fn get(State(db): State<PgPool>, auth: AuthHeaders) -> Result<Json<UserDto>, SQLError> {
    let rest_of_entity = sqlx::query_as(
        "
            SELECT created, updated, last_login
            FROM apelle_user
            WHERE id = $1
        ",
    )
    .bind(auth.id())
    .fetch_one(&db);

    let ((created, updated, last_login), roles) =
        tokio::try_join!(rest_of_entity, fetch_roles(&db, auth.id())).context(SQLSnafu)?;

    Ok(Json(UserDto {
        id: auth.id(),
        name: auth.name().to_string(),
        roles,
        created,
        updated,
        last_login,
    }))
}

async fn fetch_roles(db: &PgPool, id: Uuid) -> Result<HashSet<String>, sqlx::Error> {
    sqlx::query(
        "
            SELECT gr.name
            FROM apelle_user_global_role ugr
            INNER JOIN apelle_global_role gr
            ON ugr.global_role_id = gr.id
            WHERE ugr.user_id = $1
            ",
    )
    .bind(id)
    .map(|row| row.get(0))
    .fetch_all(db)
    .await
    .map(HashSet::from_iter)
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
        qb.push(
            "
            AND NOT EXISTS (
                SELECT 1
                FROM apelle_user
                WHERE name = ",
        )
        .push_bind(name)
        .push(" AND id != ")
        .push_bind(auth.id())
        .push(")");
    }

    let query = qb
        .push(" RETURNING  created, updated, last_login")
        .build_query_as();

    let ((created, updated, last_login), roles) = tokio::try_join!(
        async {
            Ok::<_, UpdateError>(if let Some(name) = &name {
                query
                    .fetch_optional(&db)
                    .await
                    .context(SQLSnafu)?
                    .context(ConflictSnafu { name })?
            } else {
                query.fetch_one(&db).await.context(SQLSnafu)?
            })
        },
        fetch_roles(&db, auth.id()).map(|v| Ok(v.context(SQLSnafu)?))
    )?;

    Ok(Json(UserDto {
        id: auth.id(),
        name: name.unwrap_or_else(|| auth.name().to_string()),
        roles,
        created,
        updated,
        last_login,
    }))
}

pub async fn delete(State(db): State<PgPool>, auth: AuthHeaders) -> Result<NoContent, SQLError> {
    sqlx::query(
        "
            DELETE FROM apelle_user
            WHERE id = $1
        ",
    )
    .bind(auth.id())
    .execute(&db)
    .await
    .context(SQLSnafu)?;
    Ok(NoContent)
}
