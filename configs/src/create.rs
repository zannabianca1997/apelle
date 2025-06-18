use std::collections::HashMap;

use apelle_common::common_errors::{SQLError, SQLSnafu};
use apelle_configs_dtos::{
    QueueConfig, QueueConfigCreate, QueueUserAction, QueueUserRole, QueueUserRoleCreate,
};
use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use snafu::{ResultExt as _, Snafu};
use sqlx::{
    PgPool,
    types::chrono::{DateTime, FixedOffset},
};
use textwrap_macros::unfill;
use uuid::Uuid;

use crate::config_processing::{ValidateError, validate};

#[derive(Debug, Snafu)]
pub enum CreateError {
    #[snafu(transparent)]
    SqlError { source: SQLError },
    #[snafu(transparent)]
    ValidationError { source: ValidateError },
}

impl IntoResponse for CreateError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CreateError::SqlError { source } => source.into_response(),
            CreateError::ValidationError { source } => {
                (StatusCode::BAD_REQUEST, source.to_string()).into_response()
            }
        }
    }
}

#[debug_handler(state=crate::App)]
pub async fn create(
    State(db): State<PgPool>,
    Json(config): Json<QueueConfigCreate>,
) -> Result<(StatusCode, Json<QueueConfig>), CreateError> {
    // Validate (ensure all named roles exist)
    let QueueConfigCreate {
        creator_role,
        default_role,
        banned_role,
        roles,
        autolike,
    } = validate(config)?;

    // Init transaction
    let mut tx = db.begin().await.context(SQLSnafu)?;

    sqlx::query("SET CONSTRAINTS queue_user_role_config_id_is_valid DEFERRED")
        .execute(&mut *tx)
        .await
        .context(SQLSnafu)?;

    // Choose a queue id
    let config_id = uuid::Uuid::new_v4();

    // Insert all roles
    let (names, max_likes): (Vec<&str>, Vec<i32>) = roles
        .iter()
        .map(|(name, QueueUserRoleCreate { max_likes, .. })| (name.as_str(), *max_likes as i32))
        .unzip();
    let role_ids: Vec<Uuid> = sqlx::query_scalar(
        unfill!(
            "
            INSERT INTO queue_user_role (config_id, name, max_likes) 
            SELECT $1 as config_id, * FROM UNNEST($2, $3) AS t(name, max_likes)
            RETURNING id
            "
        )
        .trim_ascii(),
    )
    .bind(config_id)
    .bind(&names)
    .bind(max_likes)
    .fetch_all(&mut *tx)
    .await
    .context(SQLSnafu)?;

    let name_mapping: HashMap<String, Uuid> = names
        .into_iter()
        .map(ToOwned::to_owned)
        .zip(role_ids)
        .collect();

    // Insert permissions
    let (role_ids, permissions): (Vec<Uuid>, Vec<QueueUserAction>) = roles
        .iter()
        .flat_map(|(name, role)| {
            role.permissions
                .iter()
                .map(|p| (name_mapping[name.as_str()], *p))
        })
        .unzip();

    sqlx::query(
        unfill!(
            "
            INSERT INTO queue_user_role_permission (role_id, permission) 
            SELECT * FROM UNNEST($1, $2) AS t(role_id, permission)
            "
        )
        .trim_ascii(),
    )
    .bind(role_ids)
    .bind(permissions)
    .execute(&mut *tx)
    .await
    .context(SQLSnafu)?;

    // Insert granted roles
    let (role_ids, can_grant): (Vec<Uuid>, Vec<Uuid>) = roles
        .iter()
        .flat_map(|(name, role)| {
            role.can_grant
                .iter()
                .map(|p| (name_mapping[name.as_str()], name_mapping[p.as_str()]))
        })
        .unzip();

    sqlx::query(
        unfill!(
            "
            INSERT INTO queue_user_grant_roles (role_id, granted_role_id) 
            SELECT * FROM UNNEST($1, $2) AS t(role_id, granted_role_id)
            "
        )
        .trim_ascii(),
    )
    .bind(role_ids)
    .bind(can_grant)
    .execute(&mut *tx)
    .await
    .context(SQLSnafu)?;

    // Insert removed roles
    let (role_ids, can_revoke): (Vec<Uuid>, Vec<Uuid>) = roles
        .iter()
        .flat_map(|(name, role)| {
            role.can_revoke
                .iter()
                .map(|p| (name_mapping[name.as_str()], name_mapping[p.as_str()]))
        })
        .unzip();

    sqlx::query(
        unfill!(
            "
            INSERT INTO queue_user_remove_roles (role_id, removed_role_id) 
            SELECT * FROM UNNEST($1, $2) AS t(role_id, removed_role_id)
            "
        )
        .trim_ascii(),
    )
    .bind(role_ids)
    .bind(can_revoke)
    .execute(&mut *tx)
    .await
    .context(SQLSnafu)?;

    // Insert config
    let (created, updated): (DateTime<FixedOffset>, DateTime<FixedOffset>) = sqlx::query_as(
        unfill!(
            "
            INSERT INTO queue_config (id, creator_role_id, default_role_id, banned_role_id, autolike) 
            VALUES ($1, $2, $3, $4, $5)
            RETURNING created, updated
            "
        )
        .trim_ascii(),
    )
    .bind(config_id)
    .bind(name_mapping[creator_role.as_str()])
    .bind(name_mapping[default_role.as_str()])
    .bind(name_mapping[banned_role.as_str()])
    .bind(autolike)
    .fetch_one(&mut *tx)
    .await
    .context(SQLSnafu)?;

    // Commit
    tx.commit().await.context(SQLSnafu)?;

    Ok((
        StatusCode::CREATED,
        Json(QueueConfig {
            id: config_id,
            created,
            updated,
            creator_role,
            default_role,
            banned_role,
            roles: roles
                .into_iter()
                .map(
                    |(
                        name,
                        QueueUserRoleCreate {
                            max_likes,
                            permissions,
                            can_grant,
                            can_revoke,
                        },
                    )| {
                        let id = name_mapping[name.as_str()];
                        (
                            name,
                            QueueUserRole {
                                id,
                                max_likes,
                                permissions,
                                can_grant,
                                can_revoke,
                            },
                        )
                    },
                )
                .collect(),
            autolike,
        }),
    ))
}
