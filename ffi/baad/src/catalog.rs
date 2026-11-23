#[diplomat::bridge]
pub mod ffi {
    use baad_shared_ffi::error::ffi::BaadError;
    use baad_shared_ffi::models::ffi::{BaadSharedDownloads, BaadSharedGlobalCatalogData};
    use baad_shared_ffi::shared::ffi::{BaadSharedBuildType, BaadSharedPlatform};

    use crate::download_adapter::resource_category;

    #[diplomat::opaque]
    pub struct BaadCatalogUrl(pub (String, bool));

    impl BaadCatalogUrl {
        pub fn url<'a>(&'a self) -> &'a str { &self.0.0 }

        pub const fn up_to_date(&self) -> bool { self.0.1 }
    }

    #[diplomat::opaque]
    pub struct BaadJapanCatalog(pub baad_native::catalog::JapanCatalog);

    impl BaadJapanCatalog {
        pub fn new(
            category: u8,
            platform: BaadSharedPlatform
        ) -> Result<Box<Self>, Box<BaadError>> {
            baad_native::catalog::JapanCatalog::new(resource_category(category), platform.into())
                .map(|catalog| Box::new(Self(catalog)))
                .map_err(baad_shared_ffi::error::error)
        }

        pub fn prepare_downloads(&self) -> Result<Box<BaadSharedDownloads>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_native::catalog::Catalog::prepare_downloads(
                &self.0
            ))
            .map(|downloads| Box::new(BaadSharedDownloads(downloads)))
        }

        pub fn get_catalog_url(&self) -> Result<Box<BaadCatalogUrl>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(self.0.get_catalog_url())
                .map(|value| Box::new(BaadCatalogUrl(value)))
        }
    }

    #[diplomat::opaque]
    pub struct BaadGlobalCatalog(pub baad_native::catalog::GlobalCatalog);

    impl BaadGlobalCatalog {
        pub fn new(
            category: u8,
            platform: BaadSharedPlatform,
            build_type: BaadSharedBuildType
        ) -> Result<Box<Self>, Box<BaadError>> {
            baad_native::catalog::GlobalCatalog::new(
                resource_category(category),
                platform.into(),
                build_type.into()
            )
            .map(|catalog| Box::new(Self(catalog)))
            .map_err(baad_shared_ffi::error::error)
        }

        pub fn prepare_downloads(&self) -> Result<Box<BaadSharedDownloads>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_native::catalog::Catalog::prepare_downloads(
                &self.0
            ))
            .map(|downloads| Box::new(BaadSharedDownloads(downloads)))
        }

        pub fn get_catalog_url(
            &self,
            version: &str
        ) -> Result<Box<BaadCatalogUrl>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(self.0.get_catalog_url(version))
                .map(|value| Box::new(BaadCatalogUrl(value)))
        }

        pub fn fetch_catalogs(
            &self,
            url: &str
        ) -> Result<Box<BaadSharedGlobalCatalogData>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(self.0.fetch_catalogs(url))
                .map(|value| Box::new(BaadSharedGlobalCatalogData(value)))
        }
    }

    #[diplomat::opaque]
    pub struct BaadChinaCatalog(pub baad_native::catalog::ChinaCatalog);

    impl BaadChinaCatalog {
        pub fn new(
            category: u8,
            platform: BaadSharedPlatform
        ) -> Result<Box<Self>, Box<BaadError>> {
            baad_native::catalog::ChinaCatalog::new(resource_category(category), platform.into())
                .map(|catalog| Box::new(Self(catalog)))
                .map_err(baad_shared_ffi::error::error)
        }

        pub fn prepare_downloads(&self) -> Result<Box<BaadSharedDownloads>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_native::catalog::Catalog::prepare_downloads(
                &self.0
            ))
            .map(|downloads| Box::new(BaadSharedDownloads(downloads)))
        }
    }
}
