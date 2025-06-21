use std::{collections::HashMap, future::ready, ops::Add, sync::Arc};

use apelle_common::{
    AuthHeaders, Reporter, TracingClient,
    common_errors::{SQLError, SQLSnafu},
};
use apelle_configs_dtos::QueueConfig;
use axum::{
    Json, debug_handler,
    extract::{Query, State},
    response::IntoResponse,
};
use chrono::{DateTime, FixedOffset};
use futures::{FutureExt, TryFutureExt};
use rand::{Rng, SeedableRng, rngs::SmallRng};
use reqwest::StatusCode;
use snafu::{ResultExt as _, Snafu};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::{
    Services,
    config::CodeConfig,
    dtos::{Config, QueueCreate},
    model::Queue,
};

#[derive(Debug, Snafu)]
pub enum CreateError {
    #[snafu(transparent)]
    SqlError {
        source: SQLError,
    },
    #[snafu(transparent)]
    ConnectionError {
        source: reqwest::Error,
    },
    ConfigNotFound,
}

impl IntoResponse for CreateError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CreateError::SqlError { source } => source.into_response(),
            CreateError::ConnectionError { source } => {
                tracing::error!("Connection error: {}", Reporter(source));
                StatusCode::BAD_GATEWAY.into_response()
            }
            CreateError::ConfigNotFound => {
                (StatusCode::NOT_FOUND, "Config not found").into_response()
            }
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreatePathParams {
    #[serde(default)]
    pub config: bool,
}

#[debug_handler(state = crate::App)]
#[utoipa::path(post, path = "/")]
/// Create a new queue
///
/// Create a new empty queue with the given code and config.
///
/// If the config is not provided, a the default config will be used.
/// If no code is provided, a random code will be generated.
pub async fn create(
    State(db): State<PgPool>,
    client: TracingClient,
    State(services): State<Arc<Services>>,
    State(code_config): State<Arc<CodeConfig>>,
    user: AuthHeaders,
    Query(CreatePathParams {
        config: return_config,
    }): Query<CreatePathParams>,
    Json(QueueCreate { code, config }): Json<QueueCreate>,
) -> Result<(StatusCode, Json<Queue>), CreateError> {
    let Services { configs_url, .. } = &*services;
    let mut tx = db.begin().await.context(SQLSnafu)?;

    let code = code
        .map(|c| ready(Ok(c)).left_future())
        .unwrap_or_else(|| gen_queue_code(&mut tx, &code_config).right_future())
        .map_err(|source| CreateError::SqlError { source });

    let config = async {
        Ok(match config {
            Config::Existing(uuid) => {
                tracing::debug!(?uuid, "Using existing queue config");

                let response = client
                    .get(configs_url.join(&uuid.to_string()).unwrap())
                    .send()
                    .await?;
                if response.status() == StatusCode::NOT_FOUND {
                    return Err(CreateError::ConfigNotFound);
                }
                response
            }
            Config::New(queue_config_create) => {
                tracing::debug!(?queue_config_create, "Creating queue config");

                client
                    .post(configs_url.clone())
                    .json(&queue_config_create)
                    .send()
                    .await?
            }
        }
        .error_for_status()?
        .json()
        .await?)
    };

    let (code, config): (_, QueueConfig) = tokio::try_join!(code, config)?;

    // Create the queue
    let (id, created, updated): (Uuid, DateTime<FixedOffset>, DateTime<FixedOffset>) =
        sqlx::query_as(
            "INSERT INTO queue (code, config_id) VALUES ($1, $2) RETURNING id, created, updated",
        )
        .bind(&code)
        .bind(config.id)
        .fetch_one(&mut *tx)
        .await
        .context(SQLSnafu)?;

    // Create the queue user
    sqlx::query("INSERT INTO queue_user (queue_id, user_id, role_id) VALUES ($1, $2, $3)")
        .bind(id)
        .bind(user.id())
        .bind(config.roles[&config.creator_role].id)
        .execute(&mut *tx)
        .await
        .context(SQLSnafu)?;

    tx.commit().await.context(SQLSnafu)?;

    Ok((
        StatusCode::CREATED,
        Json(Queue {
            id,
            current: None,
            code,
            config: if return_config {
                either::Left(config)
            } else {
                either::Right(config.id)
            },
            queue: HashMap::new(),
            created,
            updated,
        }),
    ))
}

async fn gen_queue_code(db: &mut PgConnection, config: &CodeConfig) -> Result<String, SQLError> {
    tracing::debug!("Generating queue code");

    let count: u64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM queue")
        .fetch_one(&mut *db)
        .await
        .context(SQLSnafu)? as _;

    let mut rng = SmallRng::from_os_rng();
    let mut bits = count.add(1).ilog2().add(1).max(config.min_bits);
    let alphabet_len = config.alphabet.chars().count() as u32;

    loop {
        let chars = bits.div_ceil(alphabet_len.ilog2());

        // Generate the code
        let mut code = String::with_capacity(chars as _);
        for _ in 0..chars {
            code.push(
                config
                    .alphabet
                    .chars()
                    .nth(rng.random_range(0..alphabet_len) as _)
                    .unwrap(),
            );
        }

        tracing::debug!(code, "Checking if code is free");

        if sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM queue WHERE code = $1")
            .bind(&code)
            .fetch_one(&mut *db)
            .await
            .context(SQLSnafu)?
            == 0
        {
            return Ok(code);
        }

        tracing::info!(code, "Generated code is not free, retrying");

        // Slowly increase the code complexity, ensuring that the check is sent not too many times
        bits += config.retry_bits;
    }
}
