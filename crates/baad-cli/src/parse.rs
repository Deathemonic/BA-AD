use std::process::exit;

use baad::catalog::JapanCatalog;
use baad::download::{FilterMethod, ResourceCategory, ResourceDownloader, ResourceFilter};
use baad::{Platform, file, info};
use clap::CommandFactory;
use eyre::{Result, eyre};

use crate::args::{
    Args,
    BaseDownloadArgs,
    Commands,
    GlobalDownloadArgs,
    JapanDownloadArgs,
    RegionCommands
};

pub struct CommandHandler {
    args: Args
}

impl CommandHandler {
    fn new(args: Args) -> Self { Self { args } }

    async fn handle(&self) -> Result<()> {
        if self.args.clean {
            info!("Cleaning data...");
            let data_dir = file::data_dir()?;
            file::clear_all(data_dir).await?;
            info!(success = true, "Data cleared");
        }

        match &self.args.command {
            Some(Commands::Download { region }) => self.handle_download(region).await,
            None => Ok(())
        }
    }

    async fn handle_download(&self, region: &RegionCommands) -> Result<()> {
        match region {
            RegionCommands::Global(download_args) => {
                self.execute_global_download(download_args).await
            }
            RegionCommands::Japan(download_args) => self.execute_japan_download(download_args).await
        }
    }

    async fn execute_japan_download(&self, args: &JapanDownloadArgs) -> Result<()> {
        let platform = if args.base.ios { Platform::Ios } else { Platform::Android };

        info!(platform = %platform.as_str(), "Starting Japan download");

        let catalog = JapanCatalog::new(platform)?;
        let (url, resources) = catalog.fetch_resources().await?;
        let downloads = catalog.build_downloads(&resources, &url);

        info!("Catalog fetched successfully");

        let output_dir = file::get_output_dir(Some(args.base.output.clone().into())).await?;
        let downloader =
            ResourceDownloader::new(output_dir, args.base.limit as usize, args.base.retries)
                .with_proxy(args.base.proxy.clone());

        let categories = self.resource_categories(&args.base);
        let filter = self.resource_filter(&args.base)?;

        downloader.download(downloads, &categories, filter.as_ref()).await?;

        info!("Download complete");
        Ok(())
    }

    async fn execute_global_download(&self, _args: &GlobalDownloadArgs) -> Result<()> {
        Err(eyre!("Global region not yet implemented with new architecture"))
    }

    fn resource_categories(&self, args: &BaseDownloadArgs) -> Vec<ResourceCategory> {
        let has_assets = args.assets;
        let has_tables = args.tables;
        let has_media = args.media;

        match (has_assets, has_tables, has_media) {
            (true, false, false) => vec![ResourceCategory::Assets],
            (false, true, false) => vec![ResourceCategory::Tables],
            (false, false, true) => vec![ResourceCategory::Media],
            (true, true, false) => vec![ResourceCategory::Assets, ResourceCategory::Tables],
            (true, false, true) => vec![ResourceCategory::Assets, ResourceCategory::Media],
            (false, true, true) => vec![ResourceCategory::Tables, ResourceCategory::Media],
            (true, true, true) | (false, false, false) => {
                vec![ResourceCategory::Assets, ResourceCategory::Tables, ResourceCategory::Media]
            }
        }
    }

    fn resource_filter(&self, args: &BaseDownloadArgs) -> Result<Option<ResourceFilter>> {
        let Some(filter_pattern) = &args.filter else {
            if !matches!(args.filter_method, FilterMethod::Contains) {
                let filter_method_name = format!("{:?}", args.filter_method).to_lowercase();
                return Err(eyre!(
                    "Filter method '{}' specified but no filter pattern provided. Use --filter to specify a pattern.",
                    filter_method_name
                ));
            }
            return Ok(None);
        };

        let filter = ResourceFilter::new(filter_pattern, args.filter_method.clone())?;
        Ok(Some(filter))
    }
}

pub async fn run(args: Args) -> Result<()> {
    if args.command.is_none() && !args.update && !args.clean {
        Args::command().print_help()?;
        exit(0);
    }

    let handler = CommandHandler::new(args);
    handler.handle().await
}
