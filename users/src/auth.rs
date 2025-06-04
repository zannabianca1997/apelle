use std::{collections::HashMap, sync::LazyLock};

use apelle_common::{
    AuthHeaders, Reporter,
    common_errors::{SQLError, SQLSnafu},
};
use argon2::{Argon2, PasswordVerifier as _, password_hash};
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
use route_recognizer::Router;
use snafu::{OptionExt, ResultExt, Snafu};
use sqlx::{PgPool, Row};
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
    BadDatabaseName {
        source: apelle_common::auth::InvalidName,
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
            AuthError::BadDatabaseName { source } => {
                tracing::error!(
                    "Bad database name (all names should be sanitized): {}",
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
    login_sender.try_send(id).unwrap_or_else(|_| {
        tracing::warn!("Login queue is full, last login may become inaccurate");
    });

    AuthHeaders::new(id, auth.username()).context(BadDatabaseNameSnafu)
}

/// Async worker that updated the last login of the users as they log in
///
/// This is made in a separate task to answer as quickly as possible
/// to the user
pub(crate) async fn login_updater(mut login_receiver: Receiver<Uuid>, db: PgPool) {
    while let Some(id) = login_receiver.recv().await {
        sqlx::query("UPDATE apelle_user SET last_login = NOW() WHERE id = $1")
            .bind(id)
            .execute(&db)
            .await
            .unwrap();
    }
}
