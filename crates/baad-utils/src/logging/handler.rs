use std::iter::successors;
use std::sync::Once;
use std::{error, fmt, panic};

pub use baad_shared::ConfigError;
use eyre::EyreHandler;
use tracing::error;

#[derive(Debug)]
pub struct TracingHandler;

impl TracingHandler {
    fn new() -> Self { Self }
}

impl EyreHandler for TracingHandler {
    fn debug(
        &self,
        error: &(dyn error::Error + 'static),
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {
        if f.alternate() {
            return fmt::Debug::fmt(error, f);
        }

        let mut prev_msg = error.to_string();
        error!("{}", error);

        for cause in successors(error.source(), |e| (*e).source()) {
            let cause_msg = cause.to_string();
            if cause_msg != prev_msg {
                error!("{}", cause);
            }
            prev_msg = cause_msg;
        }

        Ok(())
    }
}

pub fn install() -> Result<(), ConfigError> {
    eyre::set_hook(Box::new(|_| Box::new(TracingHandler::new())))
        .map_err(|e| ConfigError::External(Box::new(e)))?;

    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        panic::set_hook(Box::new(|panic_info| {
            let msg = panic_info
                .payload()
                .downcast_ref::<&str>()
                .map(|s| (*s).to_string())
                .or_else(|| panic_info.payload().downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "Unknown panic".to_string());

            let location = panic_info
                .location()
                .map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()))
                .unwrap_or_default();

            error!(msg = %msg, location = %location, "Panic occurred");
        }));
    });

    Ok(())
}
