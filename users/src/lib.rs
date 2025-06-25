use apelle_common::db::{SqlState, db_state_and_layer};
use argon2::Argon2;
use axum::extract::FromRef;
use config::Config;
use snafu::{ResultExt, Snafu};
use sqlx::PgPool;
use tokio::sync::mpsc::Sender;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

pub mod config;
pub mod dtos;

mod auth;
mod create;
mod me;

#[derive(Debug, Clone, FromRef)]
pub struct App {
    db: SqlState,
    password_hasher: Argon2<'static>,
    login_sender: Sender<Uuid>,
}

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {
    #[snafu(display("Cannot connect to database"))]
    Connection { source: sqlx::Error },
}

#[derive(OpenApi)]
struct AppApi;

pub async fn app(
    Config {
        db_url,
        login_queue_size,
    }: Config,
) -> Result<OpenApiRouter, MainError> {
    tracing::info!("Connecting to database");
    let (db, tx_layer) = db_state_and_layer(db_url).await.context(ConnectionSnafu)?;

    let password_hasher = Argon2::default();

    let (login_sender, login_receiver) = tokio::sync::mpsc::channel(login_queue_size);

    tokio::spawn(auth::login_updater(
        login_receiver,
        PgPool::from_ref(&db),
        login_queue_size,
    ));

    Ok(OpenApiRouter::with_openapi(AppApi::openapi())
        .nest(
            "/public",
            OpenApiRouter::new()
                .routes(routes!(create::create))
                .routes(routes!(me::get, me::patch, me::delete)),
        )
        .routes(routes!(auth::get))
        .route_layer(tx_layer)
        .with_state(App {
            db,
            password_hasher,
            login_sender,
        }))
}
