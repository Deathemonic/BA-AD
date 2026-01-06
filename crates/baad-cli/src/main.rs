mod args;
mod parse;

use args::Args;

use baad_utils::config::{init_logging, LoggingConfig};

use clap::Parser;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = LoggingConfig {
        verbose_mode: args.verbose,
        enable_debug: args.verbose,
        ..LoggingConfig::default()
    };
    init_logging(config)?;

    parse::run(args).await
}
