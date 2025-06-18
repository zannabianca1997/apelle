use std::{collections::HashMap, time::Duration};

use apelle_common::common_errors::{SQLError, SQLSnafu};
use apelle_configs_dtos::{QueueConfig, QueueUserAction, QueueUserRole};
use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers::CacheControl};
use futures::{FutureExt, StreamExt as _, TryStreamExt};
use snafu::{OptionExt, ResultExt, Snafu};
use sqlx::{PgPool, Row};
use textwrap_macros::unfill;
use uuid::Uuid;

#[derive(Debug, Snafu)]
pub enum GetError {
    #[snafu(transparent)]
    SqlError {
        source: SQLError,
    },
    NotFound,
}

impl IntoResponse for GetError {
    fn into_response(self) -> axum::response::Response {
        match self {
            GetError::SqlError { source } => source.into_response(),
            GetError::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

#[debug_handler(state=crate::App)]
pub async fn get(
    Path(id): Path<Uuid>,
    State(db): State<PgPool>,
) -> Result<(TypedHeader<CacheControl>, Json<QueueConfig>), GetError> {
    let fetch_config = sqlx::query_as(
        unfill!(
            "
            SELECT creator_role_id, default_role_id, banned_role_id, autolike, created, updated 
            FROM queue_config WHERE id = $1
            "
        )
        .trim_ascii(),
    )
    .bind(id)
    .fetch_optional(&db)
    .map(|r| r.context(SQLSnafu)?.context(NotFoundSnafu));

    let fetch_roles = sqlx::query(
        unfill!(
            "
            SELECT id, name, max_likes, ARRAY(
                SELECT permission 
                FROM queue_user_role_permission 
                WHERE role_id = queue_user_role.id
            ), ARRAY(
                SELECT granted.name 
                FROM queue_user_grant_roles 
                INNER JOIN queue_user_role granted 
                    ON granted.id = queue_user_grant_roles.granted_role_id
                WHERE role_id = queue_user_role.id
            ), ARRAY(
                SELECT removed.name 
                FROM queue_user_remove_roles 
                INNER JOIN queue_user_role removed 
                    ON removed.id = queue_user_remove_roles.removed_role_id
                WHERE role_id = queue_user_role.id
            )
            FROM queue_user_role 
            WHERE config_id = $1
            "
        )
        .trim_ascii(),
    )
    .bind(id)
    .fetch(&db)
    .map(|r| r.context(SQLSnafu).map_err(GetError::from))
    .try_fold(
        (HashMap::new(), HashMap::new()),
        async |(mut ids, mut roles), row| {
            let id: Uuid = row.get(0);
            let name: String = row.get(1);
            let max_likes = row.get::<i16, _>(2) as _;
            let permissions: Vec<QueueUserAction> = row.get(3);
            let can_grant: Vec<String> = row.get(4);
            let can_revoke: Vec<String> = row.get(5);

            ids.insert(id, name.clone());
            roles.insert(
                name,
                QueueUserRole {
                    id,
                    max_likes,
                    permissions: permissions.into_iter().collect(),
                    can_grant: can_grant.into_iter().collect(),
                    can_revoke: can_revoke.into_iter().collect(),
                },
            );

            Ok((ids, roles))
        },
    );

    let (
        (creator_role_id, default_role_id, banned_role_id, autolike, created, updated),
        (roles_ids, roles),
    ): ((Uuid, Uuid, Uuid, _, _, _), _) = tokio::try_join!(fetch_config, fetch_roles)?;

    Ok((
        // Configs are immutable, as the uuid will change when the config is updated
        TypedHeader(
            CacheControl::new()
                .with_public()
                .with_max_age(Duration::from_secs(31536000))
                .with_immutable(),
        ),
        Json(QueueConfig {
            id,
            creator_role: roles_ids.get(&creator_role_id).unwrap().clone(),
            default_role: roles_ids.get(&default_role_id).unwrap().clone(),
            banned_role: roles_ids.get(&banned_role_id).unwrap().clone(),
            roles,
            autolike,
            created,
            updated,
        }),
    ))
}
