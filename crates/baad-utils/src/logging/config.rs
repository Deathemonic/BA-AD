use std::io;
use std::sync::Arc;
use std::time::Duration;

pub use baad_core::ConfigError;
use baad_core::{DownloadObserver, NoopObserver};
use better_default::Default;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::{self};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, registry};

use crate::formatter::ConsoleFormatter;
use crate::logging::handler;
use crate::logging::writer::{AsyncMakeWriter, store_guard};
use crate::progress::{
    DownloadProgressModel,
    DownloadProgressObserver,
    ProgressMakeWriter,
    ProgressView,
    terminal
};

const PROGRESS_UPDATE_INTERVAL: Duration = Duration::from_millis(100);

#[derive(Debug, Clone, Default)]
#[default(
    enable_console: FeatureConfig::from_features().logs_enabled,
    enable_debug: FeatureConfig::from_features().debug_enabled,
    include_timestamps: true,
    enable_async_writer: true
)]
pub struct LoggingConfig {
    pub enable_console: bool,
    pub enable_json: bool,
    pub enable_debug: bool,
    pub verbose_mode: bool,
    pub include_timestamps: bool,
    pub enable_async_writer: bool
}

pub struct LoggingOutput {
    pub observer: Arc<dyn DownloadObserver>
}

#[derive(Debug, Clone)]
pub struct FeatureConfig {
    pub logs_enabled: bool,
    pub debug_enabled: bool,
    pub error_enabled: bool
}

impl FeatureConfig {
    pub fn from_features() -> Self {
        Self {
            logs_enabled: !cfg!(feature = "no_logs"),
            debug_enabled: !cfg!(any(feature = "no_debug", feature = "no_logs")),
            error_enabled: !cfg!(any(feature = "no_error", feature = "no_logs"))
        }
    }
}

fn install_error_handler(feature_config: &FeatureConfig) -> Result<(), ConfigError> {
    if feature_config.logs_enabled
        && feature_config.error_enabled
        && let Err(e) = handler::install()
    {
        return Err(ConfigError::External(Box::new(e)));
    }
    Ok(())
}

fn env_filter(verbose: bool, debug: bool) -> EnvFilter {
    match (verbose, debug) {
        (true, _) => EnvFilter::new("trace"),
        (false, true) => EnvFilter::new("debug"),
        (false, false) => EnvFilter::new("info")
    }
}

macro_rules! setup_logging {
    ($subscriber:expr, $writer:expr, $timestamps:expr, $enable_console:expr, $enable_json:expr) => {
        match ($enable_console, $enable_json) {
            (true, true) => $subscriber
                .with(
                    fmt::layer()
                        .with_writer($writer.clone())
                        .event_format(ConsoleFormatter::new().with_timestamps($timestamps))
                )
                .with(
                    fmt::layer()
                        .with_writer($writer)
                        .json()
                        .with_target(true)
                        .with_thread_ids(true)
                        .with_span_events(FmtSpan::CLOSE)
                )
                .try_init(),
            (true, false) => $subscriber
                .with(
                    fmt::layer()
                        .with_writer($writer)
                        .event_format(ConsoleFormatter::new().with_timestamps($timestamps))
                )
                .try_init(),
            (false, true) => $subscriber
                .with(
                    fmt::layer()
                        .with_writer($writer)
                        .json()
                        .with_target(true)
                        .with_thread_ids(true)
                        .with_span_events(FmtSpan::CLOSE)
                )
                .try_init(),
            (false, false) => $subscriber.try_init()
        }
    };
}

fn should_use_progress(config: &LoggingConfig) -> bool {
    terminal::is_terminal() && config.enable_console && !config.enable_json
}

fn init_with_progress(
    config: &LoggingConfig,
    feature_config: &FeatureConfig
) -> Result<LoggingOutput, ConfigError> {
    let model = DownloadProgressModel::new();
    let view = Arc::new(ProgressView::new(model, PROGRESS_UPDATE_INTERVAL));
    let observer = DownloadProgressObserver::new(Arc::clone(&view));
    let writer = ProgressMakeWriter::new(Arc::clone(&view));

    let filter =
        env_filter(config.verbose_mode, config.enable_debug && feature_config.debug_enabled);

    registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(writer)
                .with_target(false)
                .event_format(ConsoleFormatter::new().with_timestamps(config.include_timestamps))
        )
        .try_init()
        .map_err(|_| ConfigError::LoggingInitFailed)?;

    Ok(LoggingOutput {
        observer: Arc::new(observer)
    })
}

fn init_without_progress(
    config: &LoggingConfig,
    feature_config: &FeatureConfig
) -> Result<LoggingOutput, ConfigError> {
    let filter =
        env_filter(config.verbose_mode, config.enable_debug && feature_config.debug_enabled);
    let subscriber = registry().with(filter);

    let result = if config.enable_async_writer {
        let (writer, guard) = AsyncMakeWriter::new();
        let result = setup_logging!(
            subscriber,
            writer,
            config.include_timestamps,
            config.enable_console,
            config.enable_json
        );
        store_guard(guard);
        result
    } else {
        setup_logging!(
            subscriber,
            io::stdout,
            config.include_timestamps,
            config.enable_console,
            config.enable_json
        )
    };

    result.map_err(|_| ConfigError::LoggingInitFailed)?;

    Ok(LoggingOutput {
        observer: Arc::new(NoopObserver)
    })
}

pub fn init_logging(config: LoggingConfig) -> Result<LoggingOutput, ConfigError> {
    let feature_config = FeatureConfig::from_features();
    install_error_handler(&feature_config)?;

    if !feature_config.logs_enabled {
        registry().init();
        return Ok(LoggingOutput {
            observer: Arc::new(NoopObserver)
        });
    }

    if should_use_progress(&config) {
        init_with_progress(&config, &feature_config)
    } else {
        init_without_progress(&config, &feature_config)
    }
}
