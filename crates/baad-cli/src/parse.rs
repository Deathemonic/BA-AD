use std::process::exit;

use baad::catalog::{Catalog, ChinaCatalog, GlobalCatalog, JapanCatalog};
use baad::download::{FilterMethod, ResourceCategory, ResourceDownloader, ResourceFilter};
use baad::{BuildType, file, info, warn};
use clap::CommandFactory;
use eyre::{Result, eyre};

use crate::args::{
    Args,
    BaseDownloadArgs,
    ChinaDownloadArgs,
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
            return self.clean().await;
        }

        match &self.args.command {
            Some(Commands::Download { region }) => self.handle_download(region).await,
            None => Ok(())
        }
    }

    async fn clean(&self) -> Result<()> {
        info!("Cleaning data...");

        let data_dir = file::data_dir()?;
        file::clear_all(data_dir).await?;

        info!(success = true, "Data cleared");
        Ok(())
    }

    async fn handle_download(&self, region: &RegionCommands) -> Result<()> {
        match region {
            RegionCommands::Global(download_args) => self.global_download(download_args).await,
            RegionCommands::Japan(download_args) => self.japan_download(download_args).await,
            RegionCommands::China(download_args) => self.china_download(download_args).await
        }
    }

    async fn japan_download(&self, args: &JapanDownloadArgs) -> Result<()> {
        let platform = args.base.platform;
        let categories = self.resource_categories(&args.base);

        info!(platform = %platform.display_name(), "Starting Japan download");

        let catalog = JapanCatalog::new(categories, platform)?;
        self.run_download(&args.base, catalog).await
    }

    async fn global_download(&self, args: &GlobalDownloadArgs) -> Result<()> {
        let platform = args.base.platform;
        let categories = self.resource_categories(&args.base);
        let build_type = if args.teen { BuildType::Teen } else { BuildType::Standard };

        info!(platform = %platform.display_name(), "Starting Global download");

        let catalog = GlobalCatalog::new(categories, platform, build_type)?;
        self.run_download(&args.base, catalog).await
    }

    async fn china_download(&self, args: &ChinaDownloadArgs) -> Result<()> {
        let platform = args.base.platform;
        let categories = self.resource_categories(&args.base);

        info!(platform = %platform.display_name(), "Starting China download");

        let catalog = ChinaCatalog::new(categories, platform)?;
        let (_root, _resources) = catalog.fetch_resources().await?;

        warn!("China downloads are not implemented yet.");
        Ok(())
    }

    async fn run_download<C: Catalog>(&self, base: &BaseDownloadArgs, catalog: C) -> Result<()> {
        let filter = self.resource_filter(base)?;
        let output_dir = file::get_output_dir(Some(&base.output)).await?;

        let downloads = catalog.prepare_downloads().await?;

        info!("Catalog fetched successfully");

        let downloader = ResourceDownloader::builder()
            .output_dir(output_dir)
            .limit(base.limit as usize)
            .retries(base.retries)
            .maybe_proxy(base.proxy.clone())
            .build();

        downloader.download(downloads, filter.as_ref()).await?;
        Ok(())
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
