use std::{fmt::Debug, io};

use axum::Router;
use clap::Parser as _;
use figment::{Provider, providers::Serialized};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use snafu::{ResultExt, Snafu};
use tokio::{net::TcpListener, signal};
use tower_http::trace::TraceLayer;

use crate::{
    cli::{CliArgs, ProvideDefaults},
    error_reporter::Reporter,
    logging::{InitLoggingError, LoggingConfig, init_logging},
    serve::{ServeConfig, SocketConfig},
};

#[derive(Serialize, Deserialize)]
pub struct CommonConfig {
    logging: LoggingConfig,
    serve: ServeConfig,
}

impl ProvideDefaults for CommonConfig {
    fn defaults(service_name: &str, service_default_port: u16) -> impl Provider {
        Serialized::defaults(Self {
            logging: LoggingConfig::default(service_name),
            serve: ServeConfig::default(service_default_port),
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
    #[snafu(display("Cannot bind to socket {socket}"))]
    BindToSocket { socket: String, source: io::Error },
    #[snafu(display("Fatal error in serving app"))]
    RuntimeError { source: io::Error },
}

fn service_main_impl<AppConfig, AppError>(
    service_name: &'static str,
    service_default_port: u16,
    app: impl FnOnce(AppConfig) -> Result<Router, AppError>,
) -> Result<(), Error<AppError>>
where
    AppConfig: ProvideDefaults + DeserializeOwned + Debug,
    AppError: std::error::Error + 'static,
{
    let (app_config, CommonConfig { logging, serve }) = CliArgs::parse()
        .get_configuration(service_name, service_default_port)
        .context(ConfigSnafu)?;

    let log_guards = init_logging(service_name, logging).context(InitLoggingSnafu)?;

    tracing::info!("Building app");
    let app = app(app_config)?.layer(TraceLayer::new_for_http());

    tracing::info!("Starting runtime");
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context(TokioRuntimeSnafu)?
        .block_on(async {
            let tcp_listener = match &serve.socket {
                SocketConfig::Compact(socket) => TcpListener::bind(socket).await,
                SocketConfig::Large { ip, port } => TcpListener::bind((&**ip, *port)).await,
            }
            .context(BindToSocketSnafu {
                socket: serve.socket.to_string(),
            })?;

            tracing::info!("Serving app on {socket}", socket = serve.socket);

            axum::serve(tcp_listener, app)
                .with_graceful_shutdown(shutdown_signal())
                .await
                .context(RuntimeSnafu)?;

            tracing::debug!("Shutting down runtime");

            Ok::<_, Error<AppError>>(())
        })?;

    tracing::info!("Shutted down runtime");

    drop(log_guards);
    Ok(())
}

pub fn service_main<AppConfig, AppError>(
    service_name: &'static str,
    service_default_port: u16,
    app: impl FnOnce(AppConfig) -> Result<Router, AppError>,
) -> Result<(), Reporter<Error<AppError>>>
where
    AppConfig: ProvideDefaults + DeserializeOwned + Debug,
    AppError: std::error::Error + 'static,
{
    service_main_impl(service_name, service_default_port, app).map_err(Reporter)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    };
    tracing::info!("Received shutdown signal");
}
