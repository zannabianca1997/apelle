use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use apelle_common::{
    AuthHeaders, Reporter,
    db::{SqlError, SqlTx},
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
use futures::TryStreamExt;
use route_recognizer::Router;
use snafu::{OptionExt, ResultExt, Snafu};
use sqlx::{PgPool, Row};
use textwrap_macros::unfill;
use tokio::sync::mpsc::Receiver;
use utoipa::{IntoResponses, openapi};
use uuid::Uuid;

use crate::{App, auth::origins_headers::OriginHeaders};

mod origins_headers;

#[derive(Clone, Copy, Debug)]
enum EndpointAuthConfig {
    /// Does not require authentication
    Unauthenticated,

    /// Requires authentication
    Authenticated {
        /// Roles required to access this endpoint
        needed_roles: &'static [&'static str],
    },
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

fn all_methods(config: EndpointAuthConfig) -> HashMap<Method, EndpointAuthConfig> {
    let mut map = HashMap::new();
    map.insert(Method::GET, config);
    map.insert(Method::POST, config);
    map.insert(Method::PUT, config);
    map.insert(Method::PATCH, config);
    map.insert(Method::HEAD, config);
    map.insert(Method::OPTIONS, config);
    map.insert(Method::TRACE, config);
    map.insert(Method::CONNECT, config);
    map.insert(Method::DELETE, config);
    map
}

static AUTH_ROUTER: LazyLock<AuthConfig> = LazyLock::new(|| AuthConfig {
    router: {
        let mut router = Router::new();

        // User creation is open to everyone
        router.add(
            "/api/users",
            HashMap::from([(Method::POST, EndpointAuthConfig::Unauthenticated)]),
        );

        // Swagger UI and API docs is open only to developers
        for endpoint in &["/api-docs/", "/api-docs/*", "/swagger-ui/", "/swagger-ui/*"] {
            router.add(
                endpoint,
                all_methods(EndpointAuthConfig::Authenticated {
                    needed_roles: &["admin"],
                }),
            );
        }

        router
    },
    // All other endpoints require authentication, but no global role is required
    default: EndpointAuthConfig::Authenticated { needed_roles: &[] },
});

type AuthExtractor = TypedHeader<Authorization<Basic>>;

#[derive(Debug, Snafu)]
pub enum AuthError {
    #[snafu(transparent)]
    SQLError {
        source: SqlError,
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

    Forbidden,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthError::SQLError { source } => source.into_response(),
            AuthError::BadDatabasePasswordHash { source } => {
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
            AuthError::Forbidden => StatusCode::FORBIDDEN.into_response(),
        }
    }
}

impl IntoResponses for AuthError {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        [
            (
                StatusCode::INTERNAL_SERVER_ERROR.as_str().to_owned(),
                openapi::RefOr::T(openapi::Response::new("Internal Server Error")),
            ),
            (
                StatusCode::UNAUTHORIZED.as_str().to_owned(),
                openapi::RefOr::T({
                    let mut res = openapi::Response::new(concat!(
                        "Username not exists or password is incorrect, ",
                        "or missing Authorization header"
                    ));
                    res.headers.insert(
                        WWW_AUTHENTICATE.to_string(),
                        openapi::Header::builder()
                            .description(Some("Authentication challenge"))
                            .build(),
                    );
                    res
                }),
            ),
        ]
        .into_iter()
        .chain(SqlError::responses())
        .collect()
    }
}

#[debug_handler(state=crate::App)]
#[utoipa::path(get, path = "/auth",
    responses((
        status = StatusCode::OK,
        description = "User autenticated",
        headers()
    ), AuthError)
)]
/// Check user credentials.
///
/// This will return a 401 if the user is not found or the password is incorrect
/// Otherwise, it will return a 204, and decorate the response with the auth headers
/// representing the user data
pub async fn get(
    tx: SqlTx,
    State(password_hasher): State<Argon2<'static>>,
    State(login_sender): State<tokio::sync::mpsc::Sender<Uuid>>,
    OriginHeaders { uri, method }: OriginHeaders,
    auth: Result<AuthExtractor, <AuthExtractor as FromRequestParts<App>>::Rejection>,
) -> Result<(Option<AuthHeaders>, NoContent), AuthError> {
    tracing::debug!(%uri, %method, "Authenticating request");

    let needs_auth = AUTH_ROUTER.get(&uri, &method);
    let provided_auth = auth.context(MissingHeaderSnafu);

    match (needs_auth, provided_auth) {
        // Auth is required and was provided
        // Authenticating
        (EndpointAuthConfig::Authenticated { needed_roles }, Ok(auth)) => {
            let auth = authenticate(tx, password_hasher, login_sender, auth, needed_roles).await?;
            Ok((Some(auth), NoContent))
        }
        // Auth is required but was not provided
        // Failing
        (EndpointAuthConfig::Authenticated { .. }, Err(err)) => Err(err),
        // Auth is not required but was provided
        // Trying to authenticate, but not failing if they are invalid
        (EndpointAuthConfig::Unauthenticated, Ok(auth)) => {
            let auth = authenticate(tx, password_hasher, login_sender, auth, &[])
                .await
                .ok();
            Ok((auth, NoContent))
        }
        // Auth is not required and was not provided
        // Passing
        (EndpointAuthConfig::Unauthenticated, Err(_)) => Ok((None, NoContent)),
    }
}

async fn authenticate(
    mut tx: SqlTx,
    password_hasher: Argon2<'static>,
    login_sender: tokio::sync::mpsc::Sender<Uuid>,
    auth: AuthExtractor,
    needed_roles: &[&str],
) -> Result<AuthHeaders, AuthError> {
    let TypedHeader(auth) = auth;

    // Auth workflow
    let row = sqlx::query("SELECT id, password FROM apelle_user WHERE name = $1")
        .bind(auth.username())
        .fetch_optional(&mut tx)
        .await
        .map_err(SqlError::from)?
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

    // Fetching the roles
    let roles: HashSet<_> = sqlx::query_scalar::<_, String>(
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
    .fetch(&mut tx)
    .try_collect()
    .await
    .map_err(SqlError::from)?;

    for role in needed_roles {
        if !roles.contains(*role) {
            tracing::debug!(
                role,
                user = auth.username(),
                "User does not have the needed role"
            );
            return Err(AuthError::Forbidden);
        }
    }

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
