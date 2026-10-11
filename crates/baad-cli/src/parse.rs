use std::path::Path;
use std::process::exit;

use baad::catalog::{Catalog, ChinaCatalog, GlobalCatalog, JapanCatalog};
use baad::download::manifest::{self, MANIFEST_SCHEMA};
use baad::download::{
    FilterMethod,
    Manifest,
    Report,
    ResourceCategory,
    ResourceDownloader,
    ResourceFilter
};
use baad::{ASSET_BUNDLES, BuildType, MEDIA_RESOURCES, TABLE_BUNDLES, file, info, warn};
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
    const fn new(args: Args) -> Self { Self { args } }

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
        let (base, name) = match region {
            RegionCommands::Global(args) => (&args.base, "global"),
            RegionCommands::Japan(args) => (&args.base, "japan"),
            RegionCommands::China(args) => (&args.base, "china")
        };
        if let Some(path) = &base.manifest {
            return Self::manifest_download(base, name, path).await;
        }

        match region {
            RegionCommands::Global(download_args) => self.global_download(download_args).await,
            RegionCommands::Japan(download_args) => self.japan_download(download_args).await,
            RegionCommands::China(download_args) => self.china_download(download_args).await
        }
    }

    async fn japan_download(&self, args: &JapanDownloadArgs) -> Result<()> {
        let platform = args.base.platform;
        let categories = Self::resource_categories(&args.base);

        info!(platform = %platform.display_name(), "Starting Japan download");

        let catalog = JapanCatalog::new(categories, platform)?;
        Self::run_download(&args.base, catalog, "japan", BuildType::Standard).await
    }

    async fn global_download(&self, args: &GlobalDownloadArgs) -> Result<()> {
        let platform = args.base.platform;
        let categories = Self::resource_categories(&args.base);
        let build_type = if args.teen { BuildType::Teen } else { BuildType::Standard };

        info!(platform = %platform.display_name(), "Starting Global download");

        let catalog = GlobalCatalog::new(categories, platform, build_type)?;
        Self::run_download(&args.base, catalog, "global", build_type).await
    }

    async fn china_download(&self, args: &ChinaDownloadArgs) -> Result<()> {
        let platform = args.base.platform;
        let categories = Self::resource_categories(&args.base);

        info!(platform = %platform.display_name(), "Starting China download");

        let catalog = ChinaCatalog::new(categories, platform)?;
        Self::run_download(&args.base, catalog, "china", BuildType::Standard).await
    }

    async fn run_download<C: Catalog>(
        base: &BaseDownloadArgs,
        catalog: C,
        region: &str,
        build: BuildType
    ) -> Result<()> {
        let filter = Self::resource_filter(base)?;

        let (source, resources, up_to_date) = catalog.fetch_resources().await?;
        if up_to_date {
            info!("Catalog up to date");
        } else {
            info!(success = true, "Catalog fetched successfully");
        }
        let mut downloads = catalog.build_downloads(resources, &source);
        for asset in &mut downloads.assets {
            Self::categorize_path(ASSET_BUNDLES, &mut asset.path);
        }
        for table in &mut downloads.tables {
            Self::categorize_path(TABLE_BUNDLES, &mut table.path);
        }
        for media in &mut downloads.media {
            Self::categorize_path(MEDIA_RESOURCES, &mut media.path);
        }

        if let Some(path) = &base.export_manifest {
            let build: &'static str = build.into();
            let manifest = Manifest {
                schema: MANIFEST_SCHEMA,
                region: region.into(),
                platform: base.platform.as_ref().into(),
                build: build.to_lowercase(),
                version: catalog.version().await,
                source,
                resources: manifest::plan(&downloads, filter.as_ref())
            };
            manifest.save(path).await?;
            info!(
                success = true,
                files = manifest.resources.len(),
                path = %path.display(),
                "Manifest written"
            );
            return Ok(());
        }

        let downloader = Self::downloader(base).await?;
        let report = downloader.download_with_report(&downloads, filter.as_ref()).await?;
        Self::finish(base, &report).await
    }

    async fn manifest_download(base: &BaseDownloadArgs, region: &str, path: &Path) -> Result<()> {
        let manifest = Manifest::load(path).await?;
        manifest.validate(region)?;
        info!(files = manifest.resources.len(), path = %path.display(), "Downloading manifest");

        let downloader = Self::downloader(base).await?;
        let report = downloader.download_entries(&manifest.resources).await?;
        Self::finish(base, &report).await
    }

    async fn downloader(base: &BaseDownloadArgs) -> Result<ResourceDownloader> {
        let output_dir = file::get_output_dir(Some(&base.output)).await?;

        if base.boost {
            warn!("Boost is enabled this will trigger CDN rate limiting");
        }

        Ok(ResourceDownloader::builder()
            .output_dir(output_dir)
            .limit(base.limit as usize)
            .retries(base.retries)
            .maybe_proxy(base.proxy.clone())
            .http1_only(base.boost)
            .max_chunks_per_file(if base.boost { 64 } else { 16 })
            .max_concurrent_chunks(if base.boost { 32 } else { 8 })
            .chunk_threshold(if base.boost { 2 * 1024 * 1024 } else { 10 * 1024 * 1024 })
            .build())
    }

    /// Writes the report, if requested, before failing on failed files.
    async fn finish(base: &BaseDownloadArgs, report: &Report) -> Result<()> {
        if let Some(path) = &base.report {
            report.save(path).await?;
        }
        let failures = report.failures().map(ToString::to_string).collect::<Vec<_>>();
        if !failures.is_empty() {
            return Err(eyre!("{} downloads failed:\n{}", failures.len(), failures.join("\n")));
        }
        Ok(())
    }

    fn categorize_path(category_dir: &str, path: &mut String) {
        if path == category_dir
            || path.strip_prefix(category_dir).is_some_and(|path| path.starts_with('/'))
        {
            return;
        }

        path.insert(0, '/');
        path.insert_str(0, category_dir);
    }

    fn resource_categories(args: &BaseDownloadArgs) -> ResourceCategory {
        ResourceCategory::new()
            .include_if(args.assets, ResourceCategory::Assets)
            .include_if(args.tables, ResourceCategory::Tables)
            .include_if(args.media, ResourceCategory::Media)
            .or_all_if_empty()
    }

    fn resource_filter(args: &BaseDownloadArgs) -> Result<Option<ResourceFilter>> {
        let Some(filter_pattern) = &args.filter else {
            if !matches!(args.filter_method, FilterMethod::Contains) {
                let filter_method_name = format!("{:?}", args.filter_method).to_lowercase();
                return Err(eyre!(
                    "Filter method '{filter_method_name}' specified but no filter pattern provided. Use --filter to specify a pattern.",
                ));
            }
            return Ok(None);
        };

        let filter = ResourceFilter::new(filter_pattern, args.filter_method)?;
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
