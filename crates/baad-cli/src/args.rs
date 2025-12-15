use baad::download::FilterMethod;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "baad")]
#[command(about = "Blue Archive - Asset Downloader")]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Force update
    #[arg(short, long)]
    pub update: bool,

    /// Cleans the cache
    #[arg(short, long)]
    pub clean: bool,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Download game files
    Download {
        #[command(subcommand)]
        region: RegionCommands,
    },
}

#[derive(Subcommand)]
pub enum RegionCommands {
    /// Download from Global server
    Global(GlobalDownloadArgs),

    /// Download from Japan server
    Japan(JapanDownloadArgs),
}

#[derive(Parser)]
pub struct BaseDownloadArgs {
    /// Download the assetbundles
    #[arg(long)]
    pub assets: bool,

    /// Download the tablebundles
    #[arg(long)]
    pub tables: bool,

    /// Download the mediaresources
    #[arg(long)]
    pub media: bool,

    /// Output directory for the downloaded files
    #[arg(long, default_value = "./output")]
    pub output: String,

    /// Set a limit on the concurrent downloads
    #[arg(long, default_value = "10")]
    pub limit: u32,

    /// Number of retry attempts for failed downloads
    #[arg(long, default_value = "10")]
    pub retries: u32,

    /// Filter by name
    #[arg(long)]
    pub filter: Option<String>,

    /// Filter method to use
    #[arg(long, value_enum, default_value = "contains")]
    pub filter_method: FilterMethod,

    /// Proxy URL for downloads
    #[arg(long)]
    pub proxy: Option<String>,

    /// Use iOS build instead of Android
    #[arg(long)]
    pub ios: bool,
}

#[derive(Parser)]
pub struct GlobalDownloadArgs {
    #[command(flatten)]
    pub base: BaseDownloadArgs,

    /// Download Teen assets
    #[arg(long)]
    pub teen: bool,
}

#[derive(Parser)]
pub struct JapanDownloadArgs {
    #[command(flatten)]
    pub base: BaseDownloadArgs,
}