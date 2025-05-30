use std::fmt::Debug;

use clap::Parser as _;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use snafu::{ResultExt, Snafu};

use crate::{
    cli::CliArgs,
    error_reporter::Reporter,
    logging::{InitLoggingError, LoggingConfig, init_logging},
};

#[derive(Serialize, Deserialize)]
pub struct Config<AppConfig> {
    #[serde(flatten, default)]
    app: AppConfig,

    logging: LoggingConfig,
}

impl<AppConfig> Config<AppConfig> {
    pub fn default(service_name: &str) -> Self
    where
        AppConfig: Default,
    {
        Self {
            app: Default::default(),
            logging: LoggingConfig::default(service_name),
        }
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
}

fn service_main_impl<AppConfig, App, AppError>(
    service_name: &'static str,
    app: impl FnOnce(AppConfig) -> Result<App, AppError>,
) -> Result<(), Error<AppError>>
where
    AppConfig: Default + DeserializeOwned + Serialize + Debug,
    AppError: std::error::Error + 'static,
{
    let Config {
        app: app_config,
        logging,
    } = CliArgs::parse()
        .get_configuration(service_name)
        .context(ConfigSnafu)?;

    let log_guards = init_logging(service_name, logging).context(InitLoggingSnafu)?;

    tracing::info!("Building app");
    let app = app(app_config)?;

    drop(log_guards);
    Ok(())
}

pub fn service_main<AppConfig, App, AppError>(
    service_name: &'static str,
    app: impl FnOnce(AppConfig) -> Result<App, AppError>,
) -> Result<(), Reporter<Error<AppError>>>
where
    AppConfig: Default + DeserializeOwned + Serialize + Debug,
    AppError: std::error::Error + 'static,
{
    service_main_impl(service_name, app).map_err(Reporter)
}
