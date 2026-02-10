mod args;
mod parse;

use args::Args;
use baad_utils::config::{LoggingConfig, init_logging};
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
    let output = init_logging(config)?;

    parse::run(args, output.observer).await
}
