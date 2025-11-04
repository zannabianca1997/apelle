//! Common wrapper for services
//!
//! Wrap a service main with some common utilities, like configuration and logging

use std::{fmt::Debug, io};

use clap::Parser as _;
use figment::{Provider, providers::Serialized};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use snafu::{ResultExt, Snafu};

use crate::{
    cli::{CliArgs, ProvideDefaults},
    error_reporter::Reporter,
    logging::{InitLoggingError, LoggingConfig, init_logging},
};

#[derive(Serialize, Deserialize)]
pub struct CommonConfig {
    logging: LoggingConfig,
}

impl ProvideDefaults for CommonConfig {
    fn defaults(service_name: &str, _: u16) -> impl Provider {
        Serialized::defaults(Self {
            logging: LoggingConfig::default(service_name),
        })
    }
}

#[derive(Debug, Snafu)]
pub enum Error<AppError: std::error::Error + 'static> {
    #[snafu(transparent)]
    App { source: AppError },
    #[snafu(display("Error in loading configuration"))]
    Config { source: figment::Error },
    #[snafu(display("Error in initializing logging"))]
    InitLogging { source: InitLoggingError },
    #[snafu(display("Error in building the async runtime"))]
    TokioRuntime { source: io::Error },
}

fn helper_main_impl<AppConfig, AppError>(
    helper_name: &'static str,
    helper_version: &'static str,
    app: impl AsyncFnOnce(AppConfig) -> Result<(), AppError>,
) -> Result<(), Error<AppError>>
where
    AppConfig: ProvideDefaults + DeserializeOwned + Debug,
    AppError: std::error::Error + 'static,
{
    let (app_config, CommonConfig { logging }) = CliArgs::parse()
        .get_configuration(helper_name, 0)
        .context(ConfigSnafu)?;

    let log_guards = init_logging(helper_name, logging).context(InitLoggingSnafu)?;
    let full_name = format!("{helper_name}/{helper_version}").into_boxed_str();

    tracing::info!("Starting runtime");
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context(TokioRuntimeSnafu)?
        .block_on(async {
            tracing::info!(name = full_name, "Running app",);

            app(app_config).await?;

            tracing::debug!("Shutting down runtime");
            Ok::<_, Error<AppError>>(())
        })?;

    tracing::info!("Shutted down runtime");

    drop(log_guards);
    Ok(())
}

pub fn helper_main<AppConfig, AppError>(
    helper_name: &'static str,
    helper_version: &'static str,
    app: impl AsyncFnOnce(AppConfig) -> Result<(), AppError>,
) -> Result<(), Reporter<Error<AppError>>>
where
    AppConfig: ProvideDefaults + DeserializeOwned + Debug,
    AppError: std::error::Error + 'static,
{
    helper_main_impl(helper_name, helper_version, app).map_err(Reporter)
}
