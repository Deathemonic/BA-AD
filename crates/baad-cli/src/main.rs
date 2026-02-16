mod args;
mod parse;

use args::{Args, VerboseLevel};
use baad::{LoggingConfig, init_logging};
use clap::Parser;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = args.verbose.map_or_else(
        LoggingConfig::default,
        |level| LoggingConfig {
            enable_debug: true,
            enable_trace: matches!(level, VerboseLevel::Full),
            ..LoggingConfig::default()
        }
    );
    init_logging(config)?;

    parse::run(args).await
}
