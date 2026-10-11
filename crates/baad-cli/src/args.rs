use std::path::PathBuf;

use baad::Platform;
use baad::download::FilterMethod;
use clap::{Parser, Subcommand, ValueEnum};

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
    #[arg(short, long, value_name = "LEVEL", num_args = 0..=1, default_missing_value = "minimal", require_equals = true)]
    pub verbose: Option<VerboseLevel>
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum VerboseLevel {
    /// Enable debug logs
    Minimal,

    /// Enable trace logs
    Full
}

#[derive(Subcommand)]
pub enum Commands {
    /// Download game files
    Download {
        #[command(subcommand)]
        region: RegionCommands
    }
}

#[derive(Subcommand)]
pub enum RegionCommands {
    /// Download from Global server
    Global(GlobalDownloadArgs),

    /// Download from Japan server
    Japan(JapanDownloadArgs),

    /// Download from China server
    China(ChinaDownloadArgs)
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
    pub output: PathBuf,

    /// Set a limit on the concurrent downloads
    #[arg(long, default_value = "10")]
    pub limit: u32,

    /// Number of retry attempts for failed downloads
    #[arg(long, default_value = "10")]
    pub retries: u32,

    /// Filter by name
    #[arg(long)]
    pub filter: Option<String>,

    /// Filter method to use (exact, contains, regex, fuzzy, glob,
    /// contains-ignore-case, starts-with, ends-with)
    #[arg(long, default_value = "contains")]
    pub filter_method: FilterMethod,

    /// Proxy URL for downloads
    #[arg(long)]
    pub proxy: Option<String>,

    /// Aggressive downloading, this can trigger CDN rate limiting
    #[arg(long)]
    pub boost: bool,

    /// Platform to download (android, ios, windows)
    #[arg(long, default_value = "android")]
    pub platform: Platform,

    /// Write the files that would be downloaded to a JSON manifest instead of
    /// downloading them
    #[arg(long, value_name = "FILE", conflicts_with_all = ["manifest", "report"])]
    pub export_manifest: Option<PathBuf>,

    /// Download exactly the files listed in a manifest written by
    /// --export-manifest, without fetching the catalog
    #[arg(
        long,
        value_name = "FILE",
        conflicts_with_all = ["assets", "tables", "media", "filter", "platform"]
    )]
    pub manifest: Option<PathBuf>,

    /// Write the outcome of every file to a JSON report
    #[arg(long, value_name = "FILE")]
    pub report: Option<PathBuf>
}

#[derive(Parser)]
pub struct GlobalDownloadArgs {
    #[command(flatten)]
    pub base: BaseDownloadArgs,

    /// Download Teen assets
    #[arg(long, conflicts_with = "manifest")]
    pub teen: bool
}

#[derive(Parser)]
pub struct JapanDownloadArgs {
    #[command(flatten)]
    pub base: BaseDownloadArgs
}

#[derive(Parser)]
pub struct ChinaDownloadArgs {
    #[command(flatten)]
    pub base: BaseDownloadArgs
}
