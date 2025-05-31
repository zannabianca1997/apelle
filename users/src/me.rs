use apelle_common::{
    AuthHeaders,
    common_errors::{SQLError, SQLSnafu},
};
use axum::{Json, extract::State};
use snafu::ResultExt;
use sqlx::{PgPool, Row as _};

use crate::dtos::UserDto;

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

    let roles = sqlx::query(
        "
            SELECT gr.name
            FROM apelle_user_global_role ugr
            INNER JOIN apelle_global_role gr
            ON ugr.global_role_id = gr.id
            WHERE ugr.user_id = $1
            ",
    )
    .bind(auth.id())
    .map(|row| row.get(0))
    .fetch_all(&db);

    let ((created, updated, last_login), roles) =
        tokio::try_join!(rest_of_entity, roles).context(SQLSnafu)?;

    Ok(Json(UserDto {
        id: auth.id(),
        name: auth.name().to_string(),
        roles: roles.into_iter().collect(),
        created,
        updated,
        last_login,
    }))
}

pub async fn patch(State(db): State<PgPool>) -> Json<UserDto> {
    todo!()
}
