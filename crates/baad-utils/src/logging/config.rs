use std::io;
#[cfg(feature = "observer")]
use std::sync::Arc;
#[cfg(feature = "observer")]
use std::time::Duration;

#[cfg(feature = "observer")]
use baad_shared::set_observer;
use better_default::Default;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt, registry};

pub use crate::error::ConfigError;
use crate::formatter::ConsoleFormatter;
use crate::logging::handler;
use crate::logging::writer::{AsyncMakeWriter, store_guard};
#[cfg(feature = "observer")]
use crate::progress::{
    DownloadProgressHandler,
    DownloadProgressModel,
    ProgressMakeWriter,
    ProgressObserver,
    ProgressView,
    terminal
};

#[cfg(feature = "observer")]
const PROGRESS_UPDATE_INTERVAL: Duration = Duration::from_millis(100);

#[derive(Debug, Clone, Copy, Default)]
#[default(
    enable_console: true,
    enable_debug: false,
    enable_trace: false,
    include_timestamps: true,
    enable_async_writer: true,
    enable_error_handler: true,
    enable_progress: true
)]
pub struct LoggingConfig {
    pub enable_console: bool,
    pub enable_json: bool,
    pub enable_debug: bool,
    pub enable_trace: bool,
    pub include_timestamps: bool,
    pub enable_async_writer: bool,
    pub enable_error_handler: bool,
    pub enable_progress: bool
}

fn install_error_handler(config: &LoggingConfig) -> Result<(), ConfigError> {
    if config.enable_error_handler
        && let Err(e) = handler::install()
    {
        return Err(ConfigError::External(Box::new(e)));
    }
    Ok(())
}

fn env_filter(debug: bool, trace: bool) -> EnvFilter {
    match (debug, trace) {
        (true, true) => EnvFilter::new("trace"),
        (true, false) => EnvFilter::new("debug"),
        (false, _) => EnvFilter::new("info")
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

#[cfg(feature = "observer")]
fn should_use_progress(config: &LoggingConfig) -> bool {
    config.enable_progress
        && terminal::is_terminal()
        && config.enable_console
        && !config.enable_json
}

#[cfg(feature = "observer")]
fn init_with_progress<M>(config: &LoggingConfig, model: M) -> Result<(), ConfigError>
where
    M: DownloadProgressHandler + Send + Sync + 'static
{
    let view = Arc::new(ProgressView::new(model, PROGRESS_UPDATE_INTERVAL));
    let observer = ProgressObserver::new(Arc::clone(&view));
    let writer = ProgressMakeWriter::new(Arc::clone(&view));

    set_observer(Arc::new(observer));

    let filter = env_filter(config.enable_debug, config.enable_trace);

    registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(writer)
                .with_target(false)
                .event_format(ConsoleFormatter::new().with_timestamps(config.include_timestamps))
        )
        .try_init()
        .map_err(|_| ConfigError::LoggingInitFailed)
}

fn init_without_progress(config: &LoggingConfig) -> Result<(), ConfigError> {
    let filter = env_filter(config.enable_debug, config.enable_trace);
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

    result.map_err(|_| ConfigError::LoggingInitFailed)
}

#[cfg(feature = "observer")]
pub fn init_logging_with_model<M>(config: LoggingConfig, model: M) -> Result<(), ConfigError>
where
    M: DownloadProgressHandler + Send + Sync + 'static
{
    install_error_handler(&config)?;

    if should_use_progress(&config) {
        init_with_progress(&config, model)
    } else {
        init_without_progress(&config)
    }
}

#[cfg(feature = "observer")]
pub fn init_logging(config: LoggingConfig) -> Result<(), ConfigError> {
    init_logging_with_model(config, DownloadProgressModel::new())
}

#[cfg(not(feature = "observer"))]
pub fn init_logging(config: LoggingConfig) -> Result<(), ConfigError> {
    install_error_handler(&config)?;
    init_without_progress(&config)
}
