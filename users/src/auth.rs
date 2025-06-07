use std::{collections::HashMap, sync::LazyLock};

use apelle_common::{
    AuthHeaders, Reporter,
    common_errors::{SQLError, SQLSnafu},
};
use argon2::{
    Argon2, PasswordVerifier as _,
    password_hash::{self},
};
use axum::{
    debug_handler,
    extract::{FromRequestParts, State},
    http::{Method, StatusCode, header::WWW_AUTHENTICATE},
    response::{IntoResponse, NoContent},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Basic},
    typed_header::TypedHeaderRejection,
};
use futures::FutureExt;
use route_recognizer::Router;
use snafu::{OptionExt, ResultExt, Snafu};
use sqlx::{PgPool, Row};
use textwrap_macros::unfill;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

use crate::{App, auth::origins_headers::OriginHeaders};

mod origins_headers;

struct EndpointAuthConfig {
    needs_auth: bool,
    // TODO: needed profiles?
}

struct AuthConfig {
    router: Router<HashMap<Method, EndpointAuthConfig>>,
    default: EndpointAuthConfig,
}

impl AuthConfig {
    fn get(&self, uri: &str, method: &Method) -> &EndpointAuthConfig {
        self.router
            .recognize(uri)
            .ok()
            .and_then(|m| m.handler().get(method))
            .unwrap_or(&self.default)
    }
}

static AUTH_ROUTER: LazyLock<AuthConfig> = LazyLock::new(|| AuthConfig {
    router: {
        let mut router = Router::new();
        router.add(
            "/api/users",
            HashMap::from([(Method::POST, EndpointAuthConfig { needs_auth: false })]),
        );
        router
    },
    default: EndpointAuthConfig { needs_auth: true },
});

type AuthExtractor = TypedHeader<Authorization<Basic>>;

#[derive(Debug, Snafu)]
pub enum AuthError {
    #[snafu(transparent)]
    SQLError {
        source: SQLError,
    },
    UsernameNotFound,
    BadDatabasePasswordHash {
        source: password_hash::Error,
    },
    BadDatabaseNameOrRoles {
        source: apelle_common::auth::InvalidHeaders,
    },
    BadPassword {
        source: argon2::password_hash::Error,
    },
    MissingHeader {
        source: TypedHeaderRejection,
    },
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::SQLError { source } => source.into_response(),
            Self::BadDatabasePasswordHash { source } => {
                tracing::error!("Bad password hash: {}", Reporter(source));
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AuthError::UsernameNotFound
            | AuthError::BadPassword { .. }
            | AuthError::MissingHeader { .. } => (
                [(WWW_AUTHENTICATE, "Basic realm=\"apelle\"")],
                StatusCode::UNAUTHORIZED.into_response(),
            )
                .into_response(),
            AuthError::BadDatabaseNameOrRoles { source } => {
                tracing::error!(
                    "Bad database name or roles (all names or roles should be sanitized at the insertion): {}",
                    Reporter(source)
                );
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

/// Check user credentials.
///
/// This will return a 401 if the user is not found or the password is incorrect
/// Otherwise, it will return a 204, and decorate the response with the auth headers
/// representing the user data
#[debug_handler(state=crate::App)]
pub async fn get(
    State(db): State<PgPool>,
    State(password_hasher): State<Argon2<'static>>,
    State(login_sender): State<tokio::sync::mpsc::Sender<Uuid>>,
    OriginHeaders { uri, method }: OriginHeaders,
    auth: Result<AuthExtractor, <AuthExtractor as FromRequestParts<App>>::Rejection>,
) -> Result<(Option<AuthHeaders>, NoContent), AuthError> {
    tracing::debug!(%uri, %method, "Authenticating request");

    let needs_auth = AUTH_ROUTER.get(&uri, &method).needs_auth;
    let provided_auth = auth.context(MissingHeaderSnafu);

    match (needs_auth, provided_auth) {
        // Auth is required and was provided
        // Authenticating
        (true, Ok(auth)) => {
            let auth = authenticate(db, password_hasher, login_sender, auth).await?;
            Ok((Some(auth), NoContent))
        }
        // Auth is required but was not provided
        // Failing
        (true, Err(err)) => Err(err),
        // Auth is not required but was provided
        // Trying to authenticate, but not failing if they are invalid
        (false, Ok(auth)) => {
            let auth = authenticate(db, password_hasher, login_sender, auth)
                .await
                .ok();
            Ok((auth, NoContent))
        }
        // Auth is not required and was not provided
        // Passing
        (false, Err(_)) => Ok((None, NoContent)),
    }
}

async fn authenticate(
    db: PgPool,
    password_hasher: Argon2<'static>,
    login_sender: tokio::sync::mpsc::Sender<Uuid>,
    auth: AuthExtractor,
) -> Result<AuthHeaders, AuthError> {
    let TypedHeader(auth) = auth;

    // Auth workflow
    let id = async {
        let row = sqlx::query("SELECT id, password FROM apelle_user WHERE name = $1")
            .bind(auth.username())
            .fetch_optional(&db)
            .await
            .context(SQLSnafu)?
            .context(UsernameNotFoundSnafu)?;

        let password_hash =
            password_hash::PasswordHash::new(row.get(1)).context(BadDatabasePasswordHashSnafu)?;

        password_hasher
            .verify_password(auth.password().as_bytes(), &password_hash)
            .context(BadPasswordSnafu)?;

        // Passed!

        let id = row.get(0);
        tracing::info!(%id, "User logged in");
        login_sender.try_send(id).unwrap_or_else(|err| match err {
            tokio::sync::mpsc::error::TrySendError::Full(id) => {
                tracing::warn!(%id ,"Login queue is full, last login may become inaccurate");
            }
            tokio::sync::mpsc::error::TrySendError::Closed(id) => {
                // This is more severe, as the worker should never stop
                tracing::error!(%id, "Login queue is closed, id discarded");
            }
        });

        Ok(id)
    };

    // Fetching the roles
    let roles = async {
        sqlx::query_scalar::<_, String>(
            unfill!(
                "
                SELECT gr.name 
                FROM apelle_global_role gr
                JOIN apelle_user_global_role ugr ON ugr.global_role_id = gr.id
                JOIN apelle_user u ON u.id = ugr.user_id
                WHERE u.name = $1
                "
            )
            .trim_ascii(),
        )
        .bind(auth.username())
        .fetch_all(&db)
        .await
        .context(SQLSnafu)
    }
    .map(Ok::<_, AuthError>);

    let (id, roles) = tokio::try_join!(id, roles)?;
    // Unwrapping this after, so a failing role fetch do not stop early the auth
    // process (we want to report the 401, not the 500)
    let roles = roles?;

    AuthHeaders::new(id, auth.username(), &roles).context(BadDatabaseNameOrRolesSnafu)
}

/// Async worker that updated the last login of the users as they log in
///
/// This is made in a separate task to answer as quickly as possible
/// to the user
pub(crate) async fn login_updater(mut login_receiver: Receiver<Uuid>, db: PgPool, bufsize: usize) {
    let mut buffer = Vec::with_capacity(bufsize);

    while login_receiver.recv_many(&mut buffer, bufsize).await > 0 {
        if let Err(e) =
            sqlx::query("UPDATE apelle_user SET last_login = NOW() WHERE id = ANY($1::uuid[])")
                .bind(&buffer)
                .execute(&db)
                .await
        {
            tracing::error!("Failed to update last login: {}", Reporter(e));
        }

        buffer.clear();
    }
}
