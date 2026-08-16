//! `UniFFI` bindings for `baad`, mirroring its Rust API. Generated records,
//! mirrors and re-exported `baad-utils`/`baad-shared` wrappers live in
//! `shadow.rs`. Async exports run on a tokio blocking worker (`baad`'s
//! futures are not `Send`); register observers before `init_logging`.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};

use baad::api::{
    NexonClient as NativeNexonClient,
    RoStarClient as NativeRoStarClient,
    YoStarClient as NativeYoStarClient
};
use baad::catalog::{
    Catalog,
    ChinaCatalog as NativeChinaCatalog,
    GlobalCatalog as NativeGlobalCatalog,
    JapanCatalog as NativeJapanCatalog
};
use baad::cdn::{
    ChinaCdn as NativeChinaCdn,
    ChinaResources as NativeChinaResources,
    GlobalCdn as NativeGlobalCdn,
    JapanCdn as NativeJapanCdn,
    JapanResources as NativeJapanResources
};
use baad::download::{
    ResourceCategory as NativeResourceCategory,
    ResourceFilter as NativeResourceFilter,
    download_file as download_file_native
};
use baad::strategy::{ChinaStrategy, GlobalStrategy, JapanStrategy};

use super::core;

#[derive(Debug, Clone, uniffi::Record)]
pub struct ResourceCategory {
    #[uniffi(default = false)]
    pub assets: bool,
    #[uniffi(default = false)]
    pub tables: bool,
    #[uniffi(default = false)]
    pub media: bool
}

impl ResourceCategory {
    fn into_native(self) -> NativeResourceCategory {
        core::resource_category(core::category_bits(self.assets, self.tables, self.media))
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct CatalogUrl {
    pub url: String,
    pub up_to_date: bool
}

#[derive(uniffi::Object)]
pub struct ResourceFilter {
    matcher: Mutex<NativeResourceFilter>
}

impl ResourceFilter {
    fn wrap(result: Result<NativeResourceFilter, baad::FilterError>) -> Result<Self, FilterError> {
        Ok(Self {
            matcher: Mutex::new(result?)
        })
    }
}

#[uniffi::export]
impl ResourceFilter {
    #[uniffi::constructor]
    pub fn new(pattern: &str, method: FilterMethod) -> Result<Self, FilterError> {
        Self::wrap(NativeResourceFilter::new(pattern, method))
    }

    #[uniffi::constructor]
    pub fn glob(pattern: &str) -> Result<Self, FilterError> {
        Self::wrap(NativeResourceFilter::glob(pattern))
    }

    #[uniffi::constructor]
    pub fn regex(pattern: &str) -> Result<Self, FilterError> {
        Self::wrap(NativeResourceFilter::regex(pattern))
    }

    #[uniffi::constructor]
    pub fn contains(pattern: &str) -> Result<Self, FilterError> {
        Self::wrap(NativeResourceFilter::contains(pattern))
    }

    #[uniffi::constructor]
    pub fn exact(pattern: &str) -> Result<Self, FilterError> {
        Self::wrap(NativeResourceFilter::exact(pattern))
    }

    #[uniffi::constructor]
    pub fn starts_with(pattern: &str) -> Result<Self, FilterError> {
        Self::wrap(NativeResourceFilter::starts_with(pattern))
    }

    #[uniffi::constructor]
    pub fn ends_with(pattern: &str) -> Result<Self, FilterError> {
        Self::wrap(NativeResourceFilter::ends_with(pattern))
    }

    #[uniffi::constructor]
    pub fn contains_ignore_case(pattern: &str) -> Result<Self, FilterError> {
        Self::wrap(NativeResourceFilter::contains_ignore_case(pattern))
    }

    #[uniffi::constructor]
    pub fn fuzzy(pattern: &str) -> Result<Self, FilterError> {
        Self::wrap(NativeResourceFilter::fuzzy(pattern))
    }

    pub fn matches(&self, path: &str) -> bool {
        self.matcher.lock().unwrap_or_else(PoisonError::into_inner).matches(path)
    }
}

#[derive(uniffi::Object)]
pub struct JapanCatalog {
    inner: NativeJapanCatalog
}

#[uniffi::export]
impl JapanCatalog {
    #[uniffi::constructor]
    pub fn new(category: ResourceCategory, platform: Platform) -> Result<Self, CatalogError> {
        Ok(Self {
            inner: NativeJapanCatalog::new(category.into_native(), platform)?
        })
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn get_catalog_url(self: Arc<Self>) -> Result<CatalogUrl, CatalogError> {
        let (url, up_to_date) =
            run_blocking(move |handle| handle.block_on(self.inner.get_catalog_url())).await?;
        Ok(CatalogUrl { url, up_to_date })
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn prepare_downloads(self: Arc<Self>) -> Result<Downloads, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.prepare_downloads()))
            .await
            .map_err(Into::into)
    }
}

#[derive(uniffi::Object)]
pub struct GlobalCatalog {
    inner: NativeGlobalCatalog
}

#[uniffi::export]
impl GlobalCatalog {
    #[uniffi::constructor]
    pub fn new(
        category: ResourceCategory,
        platform: Platform,
        build_type: BuildType
    ) -> Result<Self, CatalogError> {
        Ok(Self {
            inner: NativeGlobalCatalog::new(category.into_native(), platform, build_type)?
        })
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn get_catalog_url(
        self: Arc<Self>,
        version: String
    ) -> Result<CatalogUrl, CatalogError> {
        let (url, up_to_date) =
            run_blocking(move |handle| handle.block_on(self.inner.get_catalog_url(&version)))
                .await?;
        Ok(CatalogUrl { url, up_to_date })
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn fetch_catalogs(
        self: Arc<Self>,
        catalog_url: String
    ) -> Result<GlobalCatalogData, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.fetch_catalogs(&catalog_url)))
            .await
            .map_err(Into::into)
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn prepare_downloads(self: Arc<Self>) -> Result<Downloads, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.prepare_downloads()))
            .await
            .map_err(Into::into)
    }
}

#[derive(uniffi::Object)]
pub struct ChinaCatalog {
    inner: NativeChinaCatalog
}

#[uniffi::export]
impl ChinaCatalog {
    #[uniffi::constructor]
    pub fn new(category: ResourceCategory, platform: Platform) -> Result<Self, CatalogError> {
        Ok(Self {
            inner: NativeChinaCatalog::new(category.into_native(), platform)?
        })
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn prepare_downloads(self: Arc<Self>) -> Result<Downloads, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.prepare_downloads()))
            .await
            .map_err(Into::into)
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct JapanResources {
    pub assets: Option<BundlePatchPackInfo>,
    pub table: Option<TableCatalog>,
    pub media: Option<MediaCatalog>
}

impl From<NativeJapanResources> for JapanResources {
    fn from(resources: NativeJapanResources) -> Self {
        Self {
            assets: resources.assets,
            table: resources.table,
            media: resources.media
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct ChinaResources {
    pub assets: Option<BundleCatalogCN>,
    pub table: Option<TableCatalogCN>,
    pub media: Option<MediaCatalogCN>
}

impl From<NativeChinaResources> for ChinaResources {
    fn from(resources: NativeChinaResources) -> Self {
        Self {
            assets: resources.assets,
            table: resources.table,
            media: resources.media
        }
    }
}

#[derive(uniffi::Object)]
pub struct JapanCdn {
    inner: NativeJapanCdn
}

#[uniffi::export]
impl JapanCdn {
    #[uniffi::constructor]
    pub const fn new(catalog_url: String, platform: Platform) -> Self {
        Self {
            inner: NativeJapanCdn::new(catalog_url, platform)
        }
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn fetch(
        self: Arc<Self>,
        category: ResourceCategory
    ) -> Result<JapanResources, CatalogError> {
        let category = category.into_native();
        let resources =
            run_blocking(move |handle| handle.block_on(self.inner.fetch(category))).await?;
        Ok(resources.into())
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn fetch_assets(self: Arc<Self>) -> Result<BundlePatchPackInfo, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.fetch_assets()))
            .await
            .map_err(Into::into)
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn fetch_table(self: Arc<Self>) -> Result<TableCatalog, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.fetch_table()))
            .await
            .map_err(Into::into)
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn fetch_media(self: Arc<Self>) -> Result<MediaCatalog, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.fetch_media()))
            .await
            .map_err(Into::into)
    }
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn japan_cdn_fetch_addressable(url: String) -> Result<JapanAddressable, CatalogError> {
    run_blocking(move |handle| handle.block_on(NativeJapanCdn::fetch_addressable(&url)))
        .await
        .map_err(Into::into)
}

#[allow(clippy::needless_pass_by_value)]
#[uniffi::export]
pub fn japan_cdn_extract_catalog_url(
    addressable: JapanAddressable
) -> Result<String, CatalogError> {
    Ok(String::from(NativeJapanCdn::extract_catalog_url(&addressable)?))
}

#[derive(uniffi::Object)]
pub struct GlobalCdn {
    inner: NativeGlobalCdn
}

#[uniffi::export]
impl GlobalCdn {
    #[uniffi::constructor]
    pub const fn new(catalog_url: String, platform: Platform) -> Self {
        Self {
            inner: NativeGlobalCdn::new(catalog_url, platform)
        }
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn fetch(self: Arc<Self>) -> Result<GlobalCatalogData, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.fetch())).await.map_err(Into::into)
    }
}

#[uniffi::export]
pub fn global_cdn_derive_base_url(resource_path: &str) -> Option<String> {
    NativeGlobalCdn::derive_base_url(resource_path).map(String::from)
}

#[derive(uniffi::Object)]
pub struct ChinaCdn {
    inner: NativeChinaCdn
}

#[uniffi::export]
impl ChinaCdn {
    #[uniffi::constructor]
    pub const fn new(
        catalog_url: String,
        platform: Platform,
        resource_version: String,
        table_version: String,
        media_version: String
    ) -> Self {
        Self {
            inner: NativeChinaCdn::new(
                catalog_url,
                platform,
                resource_version,
                table_version,
                media_version
            )
        }
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn fetch(
        self: Arc<Self>,
        category: ResourceCategory
    ) -> Result<ChinaResources, CatalogError> {
        let category = category.into_native();
        let resources =
            run_blocking(move |handle| handle.block_on(self.inner.fetch(category))).await?;
        Ok(resources.into())
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn fetch_assets(self: Arc<Self>) -> Result<BundleCatalogCN, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.fetch_assets()))
            .await
            .map_err(Into::into)
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn fetch_table(self: Arc<Self>) -> Result<TableCatalogCN, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.fetch_table()))
            .await
            .map_err(Into::into)
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn fetch_media(self: Arc<Self>) -> Result<MediaCatalogCN, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.fetch_media()))
            .await
            .map_err(Into::into)
    }
}

#[derive(uniffi::Object)]
pub struct NexonClient {
    inner: NativeNexonClient
}

#[uniffi::export]
impl NexonClient {
    #[allow(clippy::new_without_default)]
    #[uniffi::constructor]
    pub const fn new() -> Self {
        Self {
            inner: NativeNexonClient::new()
        }
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn get_addressable(
        self: Arc<Self>,
        market_config: Arc<MarketConfig>,
        version: String,
        build_number: String
    ) -> Result<GlobalAddressable, CatalogError> {
        run_blocking(move |handle| {
            handle.block_on(self.inner.get_addressable(
                &market_config.native(),
                &version,
                &build_number
            ))
        })
        .await
        .map_err(Into::into)
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn get_catalog(
        self: Arc<Self>,
        resource_path: String
    ) -> Result<GlobalCatalogData, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.get_catalog(&resource_path)))
            .await
            .map_err(Into::into)
    }
}

#[derive(uniffi::Record)]
pub struct ResourcesAsset {
    pub url: String,
    pub file: GameFile
}

#[derive(uniffi::Object)]
pub struct YoStarClient {
    inner: NativeYoStarClient
}

#[uniffi::export]
impl YoStarClient {
    #[allow(clippy::new_without_default)]
    #[uniffi::constructor]
    pub const fn new() -> Self {
        Self {
            inner: NativeYoStarClient::new()
        }
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn get_base_config(self: Arc<Self>) -> Result<GameBaseConfig, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.get_base_config()))
            .await
            .map_err(Into::into)
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn get_domain(self: Arc<Self>) -> Result<Domain, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.get_domain()))
            .await
            .map_err(Into::into)
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn get_json_config(
        self: Arc<Self>,
        version: String,
        file_path: String
    ) -> Result<GameJsonConfig, CatalogError> {
        run_blocking(move |handle| {
            handle.block_on(self.inner.get_json_config(&version, &file_path))
        })
        .await
        .map_err(Into::into)
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn get_json_data(self: Arc<Self>, url: String) -> Result<GameJsonData, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.get_json_data(&url)))
            .await
            .map_err(Into::into)
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn get_resources_asset(self: Arc<Self>) -> Result<ResourcesAsset, CatalogError> {
        let (url, file) =
            run_blocking(move |handle| handle.block_on(self.inner.get_resources_asset())).await?;
        Ok(ResourcesAsset { url, file })
    }
}

#[derive(uniffi::Object)]
pub struct RoStarClient {
    inner: NativeRoStarClient
}

#[uniffi::export]
impl RoStarClient {
    #[allow(clippy::new_without_default)]
    #[uniffi::constructor]
    pub const fn new() -> Self {
        Self {
            inner: NativeRoStarClient::new()
        }
    }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn get_state(self: Arc<Self>, version: String) -> Result<ChinaState, CatalogError> {
        run_blocking(move |handle| handle.block_on(self.inner.get_state(&version)))
            .await
            .map_err(Into::into)
    }
}

#[uniffi::export]
pub fn japan_strategy_build_asset_downloads(
    packing: BundlePatchPackInfo,
    catalog_url: &str,
    platform: Platform
) -> Vec<DownloadAsset> {
    JapanStrategy::build_asset_downloads(packing, catalog_url, platform)
}

#[uniffi::export]
pub fn japan_strategy_build_media_downloads(
    catalog: MediaCatalog,
    catalog_url: &str,
    platform: Platform
) -> Vec<DownloadMedia> {
    JapanStrategy::build_media_downloads(catalog, catalog_url, platform)
}

#[uniffi::export]
pub fn japan_strategy_build_table_downloads(
    catalog: TableCatalog,
    catalog_url: &str
) -> Vec<DownloadTable> {
    JapanStrategy::build_table_downloads(catalog, catalog_url)
}

#[uniffi::export]
pub fn global_strategy_build_downloads(
    resources: Vec<Resource>,
    base_url: &str,
    category: ResourceCategory
) -> Downloads {
    GlobalStrategy::build_downloads(resources, base_url, category.into_native())
}

#[uniffi::export]
pub fn china_strategy_build_asset_downloads(
    catalog: BundleCatalogCN,
    catalog_url: &str,
    platform: Platform
) -> Vec<DownloadAsset> {
    ChinaStrategy::build_asset_downloads(catalog, catalog_url, platform)
}

#[uniffi::export]
pub fn china_strategy_build_media_downloads(
    catalog: MediaCatalogCN,
    catalog_url: &str
) -> Vec<DownloadMedia> {
    ChinaStrategy::build_media_downloads(catalog, catalog_url)
}

#[uniffi::export]
pub fn china_strategy_build_table_downloads(
    catalog: TableCatalogCN,
    catalog_url: &str
) -> Vec<DownloadTable> {
    ChinaStrategy::build_table_downloads(catalog, catalog_url)
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct DownloaderOptions {
    pub output_dir: String,
    #[uniffi(default = 32)]
    pub limit: u32,
    #[uniffi(default = 3)]
    pub retries: u32,
    #[uniffi(default = None)]
    pub proxy: Option<String>
}

impl DownloaderOptions {
    fn to_native(&self) -> core::DownloadOptions {
        core::DownloadOptions {
            output_dir: PathBuf::from(&self.output_dir),
            limit: self.limit as usize,
            retries: self.retries,
            proxy: self.proxy.clone()
        }
    }
}

#[derive(uniffi::Object)]
pub struct ResourceDownloader {
    options: DownloaderOptions
}

#[uniffi::export]
impl ResourceDownloader {
    #[uniffi::constructor]
    pub const fn new(options: DownloaderOptions) -> Self { Self { options } }

    #[uniffi::method(async_runtime = "tokio")]
    pub async fn download(
        self: Arc<Self>,
        downloads: Downloads,
        filter: Option<Arc<ResourceFilter>>
    ) -> Result<(), CatalogError> {
        let options = self.options.to_native();

        run_blocking(move |handle| {
            let matcher = filter.as_ref().map(|filter| &filter.matcher);
            handle.block_on(core::run_download(options, downloads, matcher))
        })
        .await
        .map_err(Into::into)
    }
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn download_file(
    url: String,
    output_path: String,
    hash: Option<String>,
    retries: u32
) -> Result<(), CatalogError> {
    run_blocking(move |handle| {
        handle.block_on(download_file_native(
            &url,
            PathBuf::from(output_path).as_path(),
            hash,
            retries
        ))
    })
    .await
    .map_err(Into::into)
}

include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
