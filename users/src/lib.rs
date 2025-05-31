use argon2::Argon2;
use axum::{
    Router,
    extract::FromRef,
    routing::{get, post},
};
use config::Config;
use snafu::{ResultExt, Snafu};
use sqlx::PgPool;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

pub mod config;
pub mod dtos;

mod auth;
mod create;
mod me;

#[derive(Debug, Clone, FromRef)]
pub struct App {
    db: PgPool,
    password_hasher: Argon2<'static>,
    login_sender: Sender<Uuid>,
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

    let (login_sender, login_receiver) = tokio::sync::mpsc::channel(1);

    tokio::spawn(auth::login_updater(login_receiver, db.clone()));

    Ok(Router::new()
        .nest(
            "/public",
            Router::new()
                .route("/", post(create::create))
                .route("/me", get(me::get).patch(me::patch)),
        )
        .route("/auth", get(auth::get))
        .with_state(App {
            db,
            password_hasher,
            login_sender,
        }))
}
