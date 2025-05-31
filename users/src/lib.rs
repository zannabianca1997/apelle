use argon2::Argon2;
use axum::{Router, extract::FromRef, routing::post};
use config::Config;
use snafu::{ResultExt, Snafu};
use sqlx::PgPool;

pub mod config;
pub mod dtos;

mod create;

#[derive(Debug, Clone, FromRef)]
pub struct App {
    db: PgPool,
    password_hasher: Argon2<'static>,
}

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {
    #[snafu(display("Cannot connect to database"))]
    Connection { source: sqlx::Error },
}

pub async fn app(config: Config) -> Result<Router, MainError> {
    tracing::info!("Connecting to database");
    let db = PgPool::connect(config.db_url.as_str())
        .await
        .context(ConnectionSnafu)?;

    let password_hasher = Argon2::default();

    Ok(Router::new()
        .route("/", post(create::create))
        .with_state(App {
            db,
            password_hasher,
        }))
}
