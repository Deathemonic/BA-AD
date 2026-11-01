#[diplomat::bridge]
pub mod ffi {
    use baad_shared_ffi::error::ffi::BaadError;
    use baad_shared_ffi::models::ffi::{
        BaadSharedBundleCatalogCN,
        BaadSharedBundlePatchPackInfo,
        BaadSharedGlobalCatalogData,
        BaadSharedJapanAddressable,
        BaadSharedMediaCatalog,
        BaadSharedMediaCatalogCN,
        BaadSharedTableCatalog,
        BaadSharedTableCatalogCN
    };
    use baad_shared_ffi::shared::ffi::BaadSharedPlatform;

    #[diplomat::opaque]
    pub struct BaadJapanCdn(pub baad_native::cdn::JapanCdn);

    impl BaadJapanCdn {
        pub fn new(url: &str, platform: BaadSharedPlatform) -> Box<Self> {
            Box::new(Self(baad_native::cdn::JapanCdn::new(url.into(), platform.into())))
        }

        pub fn fetch_assets(&self) -> Result<Box<BaadSharedBundlePatchPackInfo>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(self.0.fetch_assets())
                .map(|value| Box::new(BaadSharedBundlePatchPackInfo(value)))
        }

        pub fn fetch_table(&self) -> Result<Box<BaadSharedTableCatalog>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(self.0.fetch_table())
                .map(|value| Box::new(BaadSharedTableCatalog(value)))
        }

        pub fn fetch_media(&self) -> Result<Box<BaadSharedMediaCatalog>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(self.0.fetch_media())
                .map(|value| Box::new(BaadSharedMediaCatalog(value)))
        }

        pub fn fetch_addressable(
            url: &str
        ) -> Result<Box<BaadSharedJapanAddressable>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_native::cdn::JapanCdn::fetch_addressable(url))
                .map(|value| Box::new(BaadSharedJapanAddressable(value)))
        }

        pub fn extract_catalog_url<'a>(
            addressable: &'a BaadSharedJapanAddressable
        ) -> Result<&'a str, Box<BaadError>> {
            baad_native::cdn::JapanCdn::extract_catalog_url(&addressable.0)
                .map_err(baad_shared_ffi::error::error)
        }
    }

    #[diplomat::opaque]
    pub struct BaadGlobalCdn(pub baad_native::cdn::GlobalCdn);

    impl BaadGlobalCdn {
        pub fn new(url: &str, platform: BaadSharedPlatform) -> Box<Self> {
            Box::new(Self(baad_native::cdn::GlobalCdn::new(url.into(), platform.into())))
        }

        pub fn fetch(&self) -> Result<Box<BaadSharedGlobalCatalogData>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(self.0.fetch())
                .map(|value| Box::new(BaadSharedGlobalCatalogData(value)))
        }

        pub fn derive_base_url<'a>(path: &'a str) -> Option<&'a str> {
            baad_native::cdn::GlobalCdn::derive_base_url(path)
        }
    }

    #[diplomat::opaque]
    pub struct BaadChinaCdn(pub baad_native::cdn::ChinaCdn);

    impl BaadChinaCdn {
        pub fn new(
            url: &str,
            platform: BaadSharedPlatform,
            resource_version: &str,
            table_version: &str,
            media_version: &str
        ) -> Box<Self> {
            Box::new(Self(baad_native::cdn::ChinaCdn::new(
                url.into(),
                platform.into(),
                resource_version.into(),
                table_version.into(),
                media_version.into()
            )))
        }

        pub fn fetch_assets(&self) -> Result<Box<BaadSharedBundleCatalogCN>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(self.0.fetch_assets())
                .map(|value| Box::new(BaadSharedBundleCatalogCN(value)))
        }

        pub fn fetch_table(&self) -> Result<Box<BaadSharedTableCatalogCN>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(self.0.fetch_table())
                .map(|value| Box::new(BaadSharedTableCatalogCN(value)))
        }

        pub fn fetch_media(&self) -> Result<Box<BaadSharedMediaCatalogCN>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(self.0.fetch_media())
                .map(|value| Box::new(BaadSharedMediaCatalogCN(value)))
        }
    }
}
