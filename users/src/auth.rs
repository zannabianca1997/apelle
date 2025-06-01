use apelle_common::{
    AuthHeaders, Reporter,
    common_errors::{SQLError, SQLSnafu},
};
use argon2::{Argon2, PasswordVerifier as _, password_hash};
use axum::{
    extract::State,
    http::{StatusCode, header::WWW_AUTHENTICATE},
    response::{IntoResponse, NoContent},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Basic},
    typed_header::TypedHeaderRejection,
};
use snafu::{OptionExt, ResultExt, Snafu};
use sqlx::{PgPool, Row};
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

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
pub async fn get(
    State(db): State<PgPool>,
    State(password_hasher): State<Argon2<'static>>,
    State(login_sender): State<tokio::sync::mpsc::Sender<Uuid>>,
    auth: Result<TypedHeader<Authorization<Basic>>, TypedHeaderRejection>,
) -> Result<(AuthHeaders, NoContent), AuthError> {
    let TypedHeader(auth) = auth.context(MissingHeaderSnafu)?;

    let row = sqlx::query(
        "
                SELECT id, password
                FROM apelle_user
                WHERE name = $1
            ",
    )
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

    Ok((
        AuthHeaders::new(id, auth.username()).context(BadDatabaseNameSnafu)?,
        NoContent,
    ))
}

/// Async worker that updated the last login of the users as they log in
///
/// This is made in a separate task to answer as quickly as possible
/// to the user
pub(crate) async fn login_updater(mut login_receiver: Receiver<Uuid>, db: PgPool) {
    while let Some(id) = login_receiver.recv().await {
        sqlx::query(
            "
                UPDATE apelle_user
                SET last_login = NOW()
                WHERE id = $1
            ",
        )
        .bind(id)
        .execute(&db)
        .await
        .unwrap();
    }
}
