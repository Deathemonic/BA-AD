use crate::async_writer::{store_guard, AsyncMakeWriter};
use crate::error::ConfigError;
use crate::formatter::ConsoleFormatter;

use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

#[derive(Debug, Clone)]
pub struct LoggingConfig {
    pub enable_console: bool,
    pub enable_json: bool,
    pub enable_debug: bool,
    pub verbose_mode: bool,
    pub include_timestamps: bool,
    pub enable_async_writer: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        let feature_config = FeatureConfig::from_features();

        Self {
            enable_console: feature_config.logs_enabled,
            enable_json: false,
            enable_debug: feature_config.debug_enabled,
            verbose_mode: false,
            include_timestamps: true,
            enable_async_writer: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FeatureConfig {
    pub logs_enabled: bool,
    pub debug_enabled: bool,
    pub error_enabled: bool,
}

impl FeatureConfig {
    pub fn from_features() -> Self {
        Self {
            logs_enabled: !cfg!(feature = "no_logs"),
            debug_enabled: !cfg!(any(feature = "no_debug", feature = "no_logs")),
            error_enabled: !cfg!(any(feature = "no_error", feature = "no_logs")),
        }
    }
}

pub fn init_logging(config: LoggingConfig) -> Result<(), ConfigError> {
    let feature_config = FeatureConfig::from_features();

    if feature_config.logs_enabled
        && feature_config.error_enabled
        && let Err(e) = crate::error::install()
    {
        return Err(ConfigError::External(Box::new(e)));
    }

    if !feature_config.logs_enabled {
        tracing_subscriber::registry().init();
        return Ok(());
    }

    let env_filter = match (
        config.verbose_mode,
        config.enable_debug && feature_config.debug_enabled,
    ) {
        (true, _) => EnvFilter::new("trace"),
        (false, true) => EnvFilter::new("debug"),
        (false, false) => EnvFilter::new("info"),
    };

    if config.enable_async_writer {
        let (async_writer, guard) = AsyncMakeWriter::new();

        macro_rules! console_layer {
            () => {
                fmt::layer().with_writer(async_writer.clone()).event_format(
                    ConsoleFormatter::new().with_timestamps(config.include_timestamps),
                )
            };
        }

        macro_rules! json_layer {
            () => {
                fmt::layer()
                    .with_writer(async_writer.clone())
                    .json()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_span_events(FmtSpan::CLOSE)
            };
        }

        let subscriber = tracing_subscriber::registry().with(env_filter);

        let result = match (config.enable_console, config.enable_json) {
            (true, true) => subscriber
                .with(console_layer!())
                .with(json_layer!())
                .try_init(),
            (true, false) => subscriber.with(console_layer!()).try_init(),
            (false, true) => subscriber.with(json_layer!()).try_init(),
            (false, false) => subscriber.try_init(),
        };

        result.map_err(|_| ConfigError::LoggingInitFailed)?;
        store_guard(guard);
    } else {
        macro_rules! console_layer {
            () => {
                fmt::layer().event_format(
                    ConsoleFormatter::new().with_timestamps(config.include_timestamps),
                )
            };
        }

        macro_rules! json_layer {
            () => {
                fmt::layer()
                    .json()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_span_events(FmtSpan::CLOSE)
            };
        }

        let subscriber = tracing_subscriber::registry().with(env_filter);

        let result = match (config.enable_console, config.enable_json) {
            (true, true) => subscriber
                .with(console_layer!())
                .with(json_layer!())
                .try_init(),
            (true, false) => subscriber.with(console_layer!()).try_init(),
            (false, true) => subscriber.with(json_layer!()).try_init(),
            (false, false) => subscriber.try_init(),
        };

        result.map_err(|_| ConfigError::LoggingInitFailed)?;
    }

    Ok(())
}

pub fn init_logging_default() -> Result<(), ConfigError> {
    init_logging(LoggingConfig::default())
}
