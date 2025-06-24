use std::sync::Arc;

use apelle_common::{
    AuthHeaders,
    common_errors::{SQLError, SQLSnafu},
};
use apelle_configs_dtos::{QueueConfig, QueueUserAction, QueueUserRole};
use axum::{
    Extension, debug_middleware,
    extract::{Path, Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use snafu::{ResultExt, Snafu};
use sqlx::PgPool;
use textwrap_macros::unfill;
use uuid::Uuid;

use crate::QueuePathParams;

#[derive(Debug, Clone)]
pub struct QueueUser {
    user: AuthHeaders,
    auto_like: bool,
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
}

#[derive(Debug, Snafu)]
pub enum ExtractQueueUserError {
    #[snafu(transparent)]
    SqlError { source: SQLError },
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
pub async fn extract_queue_user(
    State(db): State<PgPool>,
    Extension(config): Extension<Arc<QueueConfig>>,
    user: AuthHeaders,
    Path(QueuePathParams { id: queue_id }): Path<QueuePathParams>,
    mut request: Request,
    next: Next,
) -> Result<Response, ExtractQueueUserError> {
    let default_role_id: Uuid = config.roles.get(&config.default_role).unwrap().id;
    let autolike_default = config.autolike;

    // Create the user, or get the existing one setup
    let (role_id, autolike_override): (Uuid, Option<bool>) = sqlx::query_as(unfill!(
        "
        INSERT INTO queue_user (queue_id, user_id, role_id)
        VALUES ($1, $2, $3)
        ON CONFLICT (queue_id, user_id) DO UPDATE SET last_seen = NOW()
        RETURNING role_id, autolike
        "
    ))
    .bind(queue_id)
    .bind(user.id())
    .bind(default_role_id)
    .fetch_one(&db)
    .await
    .context(SQLSnafu)?;

    let user = QueueUser {
        user,
        auto_like: autolike_override.unwrap_or(autolike_default),
        role: config
            .roles
            .values()
            .find(|r| r.id == role_id)
            .unwrap()
            .clone(),
    };

    request.extensions_mut().insert(Arc::new(user));

    Ok(next.run(request).await)
}
