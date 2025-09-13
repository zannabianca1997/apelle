use std::{collections::HashMap, time::Duration};

use apelle_common::db::{SqlError, SqlTx};
use apelle_configs_dtos::{QueueConfig, QueueUserAction, QueueUserRole};
use axum::{Json, debug_handler, extract::Path, http::StatusCode, response::IntoResponse};
use axum_extra::{TypedHeader, headers::CacheControl};
use futures::TryStreamExt;
use snafu::{OptionExt, Snafu};
use utoipa::{
    IntoResponses,
    openapi::{self, RefOr},
};
use uuid::Uuid;

#[derive(Debug, Snafu)]
pub enum GetError {
    #[snafu(transparent)]
    SqlError {
        source: SqlError,
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

impl IntoResponses for GetError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [(
            StatusCode::NOT_FOUND.as_str().to_string(),
            RefOr::T(openapi::Response::new("Queue not found")),
        )]
        .into_iter()
        .chain(SqlError::responses())
        .collect()
    }
}

#[debug_handler(state=crate::App)]
#[utoipa::path(get, path = "/queues/{id}", responses((status = StatusCode::OK, description = "Queue config", content_type = "application/json", body = QueueConfig), GetError))]
/// Returns a queue config
///
/// Queue configs are immutable. When a queue change its config, a new config is
/// created instead of changing the old one. This makes the result of this call
/// cachable.
pub async fn get(
    Path(id): Path<Uuid>,
    mut tx: SqlTx,
) -> Result<(TypedHeader<CacheControl>, Json<QueueConfig>), GetError> {
    let record = sqlx::query!(
        "
        SELECT creator_role_id, default_role_id, banned_role_id, autolike, created, updated 
        FROM queue_config WHERE id = $1
        ",
        id
    )
    .fetch_optional(&mut tx)
    .await
    .map_err(SqlError::from)?
    .context(NotFoundSnafu)?;

    let (roles_ids, roles) = sqlx::query!(
        r#"
        SELECT id, name, max_likes, 
        ARRAY(
            SELECT permission 
            FROM queue_user_role_permission 
            WHERE role_id = queue_user_role.id
        ) AS "permissions!: Vec<QueueUserAction>",
        ARRAY(
            SELECT granted.name 
            FROM queue_user_grant_roles 
            INNER JOIN queue_user_role granted 
                ON granted.id = queue_user_grant_roles.granted_role_id
            WHERE role_id = queue_user_role.id
        ) AS "can_grant!", 
        ARRAY(
            SELECT removed.name 
            FROM queue_user_remove_roles 
            INNER JOIN queue_user_role removed 
                ON removed.id = queue_user_remove_roles.removed_role_id
            WHERE role_id = queue_user_role.id
        ) AS "can_revoke!"
        FROM queue_user_role 
        WHERE config_id = $1
        "#,
        id
    )
    .fetch(&mut tx)
    .try_fold(
        (HashMap::new(), HashMap::new()),
        async |(mut ids, mut roles), row| {
            ids.insert(row.id, row.name.clone());
            roles.insert(
                row.name,
                QueueUserRole {
                    id: row.id,
                    max_likes: row.max_likes as _,
                    permissions: row.permissions.into_iter().collect(),
                    can_grant: row.can_grant.into_iter().collect(),
                    can_revoke: row.can_revoke.into_iter().collect(),
                },
            );

            Ok((ids, roles))
        },
    )
    .await
    .map_err(SqlError::from)?;

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
            creator_role: roles_ids.get(&record.creator_role_id).unwrap().clone(),
            default_role: roles_ids.get(&record.default_role_id).unwrap().clone(),
            banned_role: roles_ids.get(&record.banned_role_id).unwrap().clone(),
            roles,
            autolike: record.autolike,
            created: record.created.into(),
            updated: record.updated.into(),
        }),
    ))
}
