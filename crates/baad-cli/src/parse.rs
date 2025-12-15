use crate::cli::args::{
    Args, BaseDownloadArgs, Commands, GlobalDownloadArgs, JapanDownloadArgs, RegionCommands,
};

use baad::apk::{ApkExtractor, ApkFetcher};
use baad::catalog::{CatalogFetcher, CatalogParser};
use baad::download::{FilterMethod, ResourceCategory, ResourceDownloadBuilder, ResourceFilter};
use baad::helpers::{ApkError, BuildType, Platform, ServerConfig, ServerRegion};

use baad_core::{file, info};
use clap::CommandFactory;
use eyre::{eyre, Result};
use std::rc::Rc;

pub struct CommandHandler {
    args: Args,
}

impl CommandHandler {
    fn new(args: Args) -> Result<Self> {
        Ok(Self { args })
    }

    async fn handle(&self) -> Result<()> {
        if self.args.clean {
            info!("Cleaning data...");

            let data_dir = file::data_dir()?;
            file::clear_all(&data_dir).await?;

            info!(success = true, "Data cleared");
        }

        match &self.args.command {
            Some(Commands::Download { region }) => self.handle_download(region).await,
            None => {
                if self.args.update {
                    self.handle_update().await?;
                }
                Ok(())
            }
        }
    }

    async fn handle_download(&self, region: &RegionCommands) -> Result<()> {
        match region {
            RegionCommands::Global(download_args) => {
                self.execute_global_download(download_args).await
            }
            RegionCommands::Japan(download_args) => {
                self.execute_japan_download(download_args).await
            }
        }
    }

    async fn handle_update(&self) -> Result<()> {
        info!("Forcing update...");

        let server_config = ServerConfig::new(ServerRegion::Japan, None, None)?;
        let apk_fetcher = ApkFetcher::new(server_config.clone())?;

        apk_fetcher.download_apk(true).await?;

        Ok(())
    }

    async fn execute_global_download(&self, args: &GlobalDownloadArgs) -> Result<()> {
        let platform = if args.base.ios {
            Some(Platform::Ios)
        } else {
            None
        };
        let build_type = if args.teen {
            Some(BuildType::Teen)
        } else {
            None
        };

        let server_config = ServerConfig::new(ServerRegion::Global, platform, build_type)?;
        let apk_fetcher = ApkFetcher::with_proxy(server_config.clone(), args.base.proxy.clone())?;

        let should_process_catalogs = self.handle_global(&apk_fetcher).await?;

        if should_process_catalogs {
            apk_fetcher.check_version().await?;
            self.process_catalogs(&server_config, &apk_fetcher).await?;
        }

        if !should_process_catalogs {
            info!("Catalog files exist and are up to date, skipping catalog processing...");
        }

        self.download_resources(&server_config, &args.base).await?;

        Ok(())
    }

    async fn execute_japan_download(&self, args: &JapanDownloadArgs) -> Result<()> {
        let platform = if args.base.ios {
            Some(Platform::Ios)
        } else {
            None
        };
        let build_type = None;

        let server_config = ServerConfig::new(ServerRegion::Japan, platform, build_type)?;
        let apk_fetcher = ApkFetcher::with_proxy(server_config.clone(), args.base.proxy.clone())?;

        let should_process_catalogs = {
            let should_process = self.handle_japan(&apk_fetcher).await?;

            if should_process {
                apk_fetcher.download_apk(self.args.update).await?;

                let apk_extractor = ApkExtractor::new(server_config.clone())?;
                apk_extractor.extract_data()?;
            }

            should_process
        };

        if should_process_catalogs {
            apk_fetcher.check_version().await?;
            self.process_catalogs(&server_config, &apk_fetcher).await?;
        }

        if !should_process_catalogs {
            info!("Catalog files exist and are up to date, skipping catalog processing...");
        }

        self.download_resources(&server_config, &args.base).await?;

        Ok(())
    }

    async fn handle_japan(&self, apk_fetcher: &ApkFetcher) -> Result<bool, ApkError> {
        let data_path = file::get_data_path("data")?;
        let catalog_path = file::get_data_path("catalog/japan")?;

        let data_empty = file::is_dir_empty(&data_path).await?;
        let catalogs_empty = file::is_dir_empty(&catalog_path).await?;

        if data_empty || catalogs_empty || self.args.update {
            return Ok(true);
        }

        apk_fetcher.needs_catalog_update().await
    }

    async fn handle_global(&self, apk_fetcher: &ApkFetcher) -> Result<bool, ApkError> {
        let catalog_path = file::get_data_path("catalog/global")?;

        let catalogs_empty = file::is_dir_empty(&catalog_path).await?;

        if catalogs_empty || self.args.update {
            return Ok(true);
        }

        apk_fetcher.needs_catalog_update().await
    }

    async fn process_catalogs(
        &self,
        server_config: &Rc<ServerConfig>,
        apk_fetcher: &ApkFetcher,
    ) -> Result<()> {
        let catalog_fetcher = CatalogFetcher::new(server_config.clone(), apk_fetcher.clone())?;
        let catalog_parser = CatalogParser::new(server_config.clone())?;

        catalog_fetcher.get_addressable().await?;
        catalog_fetcher.get_catalogs().await?;
        catalog_parser.process_catalogs().await?;

        Ok(())
    }

    async fn download_resources(
        &self,
        server_config: &Rc<ServerConfig>,
        args: &BaseDownloadArgs,
    ) -> Result<()> {
        let resource_downloader = ResourceDownloadBuilder::new(server_config.clone())?
            .limit(args.limit as u64)
            .retries(args.retries)
            .output(Some(args.output.clone().into()))
            .proxy(args.proxy.clone())
            .build()
            .await?;

        let resource_category = self.resource_category(args);
        let resource_filter = self.resource_filter(args)?;
        resource_downloader
            .download(resource_category, resource_filter)
            .await?;

        Ok(())
    }

    fn resource_category(&self, args: &BaseDownloadArgs) -> ResourceCategory {
        let has_assets = args.assets;
        let has_tables = args.tables;
        let has_media = args.media;

        match (has_assets, has_tables, has_media) {
            (true, false, false) => ResourceCategory::Assets,
            (false, true, false) => ResourceCategory::Tables,
            (false, false, true) => ResourceCategory::Media,
            (true, true, true) => ResourceCategory::All,
            (false, false, false) => ResourceCategory::All,
            (true, true, false) => {
                ResourceCategory::multiple(vec![ResourceCategory::Assets, ResourceCategory::Tables])
            }
            (true, false, true) => {
                ResourceCategory::multiple(vec![ResourceCategory::Assets, ResourceCategory::Media])
            }
            (false, true, true) => {
                ResourceCategory::multiple(vec![ResourceCategory::Tables, ResourceCategory::Media])
            }
        }
    }

    fn resource_filter(&self, args: &BaseDownloadArgs) -> Result<Option<ResourceFilter>> {
        let Some(filter_pattern) = &args.filter else {
            if !matches!(args.filter_method, FilterMethod::Contains) {
                let filter_method_name = format!("{:?}", args.filter_method).to_lowercase();
                return Err(eyre!(format!(
                    "Filter method '{}' specified but no filter pattern provided. Use --filter to specify a pattern.",
                    filter_method_name
                )));
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
        std::process::exit(0);
    }

    let handler = CommandHandler::new(args)?;
    handler.handle().await
}