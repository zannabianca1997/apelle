use std::sync::Arc;

use apelle_common::{
    AuthHeaders,
    db::{SqlError, SqlTx},
};
use apelle_configs_dtos::{QueueConfig, QueueUserAction, QueueUserRole};
use axum::{
    Extension, debug_middleware,
    extract::{Path, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};
use snafu::Snafu;
use textwrap_macros::unfill;
use tracing::instrument;
use uuid::Uuid;

use crate::QueuePathParams;

#[derive(Debug, Clone)]
pub struct QueueUser {
    user: AuthHeaders,
    auto_like: bool,
    likes: u16,
    role: QueueUserRole,
}

impl QueueUser {
    pub fn user(&self) -> &AuthHeaders {
        &self.user
    }

    pub fn auto_like(&self) -> bool {
        self.auto_like
    }

    pub fn role(&self) -> &QueueUserRole {
        &self.role
    }

    pub fn can(&self, value: QueueUserAction) -> bool {
        self.role.permissions.contains(&value)
    }

    pub fn id(&self) -> Uuid {
        self.user.id()
    }

    /// The number of likes this user has given in total
    pub fn likes(&self) -> u16 {
        self.likes
    }
}

#[derive(Debug, Snafu)]
pub enum ExtractQueueUserError {
    #[snafu(transparent)]
    SqlError { source: SqlError },
}

impl IntoResponse for ExtractQueueUserError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ExtractQueueUserError::SqlError { source } => source.into_response(),
        }
    }
}

/// Extract data about the queue user
///
/// This will extract the user from the request, and get the role from the
/// database and the config. If this is the first time the user is seen,
/// it will also create the user in the database with the default role.
///
/// The resulting data are added to the request as extensions.
#[debug_middleware(state = crate::App)]
#[instrument(skip_all, fields(
    %queue_id, user_id =% user.id()
))]
pub async fn extract_queue_user(
    mut tx: SqlTx,
    Extension(config): Extension<Arc<QueueConfig>>,
    user: AuthHeaders,
    Path(QueuePathParams { id: queue_id }): Path<QueuePathParams>,
    mut request: Request,
    next: Next,
) -> Result<Response, ExtractQueueUserError> {
    let default_role_id: Uuid = config.roles.get(&config.default_role).unwrap().id;
    let autolike_default = config.autolike;

    // Create the user, or get the existing one setup
    let (role_id, autolike_override, likes): (Uuid, Option<bool>, i16) = sqlx::query_as(unfill!(
        "
        WITH upsert_queue_user AS (
            INSERT INTO queue_user (queue_id, user_id, role_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (queue_id, user_id) DO UPDATE SET last_seen = NOW()
            RETURNING queue_id, user_id, role_id, autolike
        )
        SELECT
            u.role_id,
            u.autolike,
            COALESCE(SUM(l.count)::smallint, 0::smallint) AS total_likes
        FROM
            upsert_queue_user u
        LEFT JOIN
            likes l ON u.queue_id = l.queue_id AND u.user_id = l.user_id
        GROUP BY
            u.queue_id, u.user_id, u.role_id, u.autolike;
        "
    ))
    .bind(queue_id)
    .bind(user.id())
    .bind(default_role_id)
    .fetch_one(&mut tx)
    .await
    .map_err(SqlError::from)?;

    let user = QueueUser {
        user,
        auto_like: autolike_override.unwrap_or(autolike_default),
        role: config
            .roles
            .values()
            .find(|r| r.id == role_id)
            .unwrap()
            .clone(),
        likes: likes as _,
    };

    request.extensions_mut().insert(Arc::new(user));

    drop(tx);

    Ok(next.run(request).await)
}
