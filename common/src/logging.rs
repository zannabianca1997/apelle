use std::{collections::HashMap, io::stdout, path::PathBuf};

use ::serde::{Deserialize, Serialize};
use figment::value::magic::RelativePathBuf;
use serde_with::{DisplayFromStr, serde_as};
use snafu::{ResultExt, Snafu};
use tracing::{Subscriber, level_filters::LevelFilter};
use tracing_appender::rolling::Rotation;
use tracing_subscriber::{
    Layer,
    filter::Targets,
    layer::SubscriberExt,
    registry::LookupSpan,
    util::{SubscriberInitExt, TryInitError},
};

const DEFAULT_LOG_DIR: &str = "./logs";

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum SerdeRotation {
    Minutely,
    Hourly,
    #[default]
    Daily,
    Never,
}

impl From<SerdeRotation> for Rotation {
    fn from(value: SerdeRotation) -> Self {
        match value {
            SerdeRotation::Minutely => Rotation::MINUTELY,
            SerdeRotation::Hourly => Rotation::HOURLY,
            SerdeRotation::Daily => Rotation::DAILY,
            SerdeRotation::Never => Rotation::NEVER,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggingConfig {
    #[serde(default)]
    file: Option<FileLogging>,
    #[serde(default)]
    console: Option<ConsoleLogging>,
}
impl LoggingConfig {
    pub fn default(service_name: &str) -> Self {
        Self {
            file: Some(FileLogging::default(service_name)),
            console: Some(ConsoleLogging::default(service_name)),
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum TargetsConfig {
    Level(#[serde_as(as = "DisplayFromStr")] LevelFilter),
    Nested(HashMap<String, TargetsConfig>),
}

impl TargetsConfig {
    fn extend(self, mut targets: Targets, root: String) -> Targets {
        match self {
            TargetsConfig::Level(level) => targets.with_target(root, level),
            TargetsConfig::Nested(nested) => {
                for (name, config) in nested {
                    targets = config.extend(targets, format!("{root}::{name}"));
                }
                targets
            }
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct FileLogging {
    /// Directory where the rolling logs will be stored
    ///
    /// None disables the rolling logs
    dir: RelativePathBuf,

    /// Rotation strategy of the rolling logs
    rotation: SerdeRotation,

    /// Prefix of the rolling log files
    prefix: String,

    /// Root level filter of the logs
    #[serde_as(as = "DisplayFromStr")]
    level: LevelFilter,

    /// List of additional targets with a specific level filter
    targets: HashMap<String, TargetsConfig>,
}

impl FileLogging {
    pub fn default(service_name: &str) -> Self {
        Self {
            dir: RelativePathBuf::from(PathBuf::from(DEFAULT_LOG_DIR).join(service_name)),
            prefix: format!("{service_name}.log"),
            rotation: SerdeRotation::Daily,
            level: if cfg!(debug_assertions) {
                LevelFilter::DEBUG
            } else {
                LevelFilter::INFO
            },
            targets: HashMap::new(),
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct ConsoleLogging {
    /// Root level filter of the logs
    #[serde_as(as = "DisplayFromStr")]
    level: LevelFilter,

    /// List of additional targets with a specific level filter
    targets: HashMap<String, TargetsConfig>,
}

impl ConsoleLogging {
    pub fn default(_service_name: &str) -> Self {
        Self {
            level: if cfg!(debug_assertions) {
                LevelFilter::DEBUG
            } else {
                LevelFilter::INFO
            },
            targets: HashMap::new(),
        }
    }
}

pub struct TracingGuard {
    _file_worker_guard: Option<tracing_appender::non_blocking::WorkerGuard>,
}

#[derive(Debug, Snafu)]
pub enum InitLoggingError {
    #[snafu(display("Cannot create log directory `{}`", dir.display()))]
    CannotCreateLogDir {
        dir: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Cannot initialize tracing subscriber"))]
    InitSubscriber { source: TryInitError },
}

pub fn init_logging(
    service_name: &'static str,
    LoggingConfig { file, console }: LoggingConfig,
) -> Result<TracingGuard, InitLoggingError> {
    let (file_writer, file_worker_guard) = file
        .map(|f| file_logging(service_name, f))
        .transpose()?
        .unzip();

    let console_writer = console
        .map(|c| console_logging(service_name, c))
        .transpose()?;

    tracing_subscriber::registry()
        .with(file_writer)
        .with(console_writer)
        .try_init()
        .context(InitSubscriberSnafu)?;

    tracing::info!("Initialized logging");
    Ok(TracingGuard {
        _file_worker_guard: file_worker_guard,
    })
}

fn file_logging<S: Subscriber + for<'a> LookupSpan<'a>>(
    _service_name: &'static str,
    FileLogging {
        dir,
        prefix,
        rotation,
        level,
        targets,
    }: FileLogging,
) -> Result<(impl Layer<S>, tracing_appender::non_blocking::WorkerGuard), InitLoggingError> {
    let dir = dir.relative();

    std::fs::create_dir_all(&dir).context(CannotCreateLogDirSnafu { dir: &dir })?;

    let (file_writer, guard) = tracing_appender::non_blocking(
        tracing_appender::rolling::RollingFileAppender::new(rotation.into(), dir, prefix),
    );

    let mut all_targets = Targets::new().with_default(level);
    for (root, config) in targets.into_iter() {
        all_targets = config.extend(all_targets, root);
    }

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc_3339())
        .with_writer(file_writer)
        .with_ansi(false)
        .with_target(true)
        .with_level(true)
        .with_filter(all_targets);

    Ok((file_subscriber, guard))
}

fn console_logging<S: Subscriber + for<'a> LookupSpan<'a>>(
    _service_name: &'static str,
    ConsoleLogging { level, targets }: ConsoleLogging,
) -> Result<impl Layer<S>, InitLoggingError> {
    let mut all_targets = Targets::new().with_default(level);
    for (root, config) in targets.into_iter() {
        all_targets = config.extend(all_targets, root);
    }

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc_3339())
        .with_writer(stdout)
        .with_ansi(atty::is(atty::Stream::Stdout))
        .with_target(true)
        .with_level(true)
        .with_filter(all_targets);

    Ok(file_subscriber)
}
