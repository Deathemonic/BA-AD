#[diplomat::bridge]
pub mod ffi {
    use baad_shared_ffi::models::ffi::{
        BaadSharedBundleCatalogCN,
        BaadSharedBundlePatchPackInfo,
        BaadSharedDownloads,
        BaadSharedGlobalCatalogData,
        BaadSharedMediaCatalog,
        BaadSharedMediaCatalogCN,
        BaadSharedTableCatalog,
        BaadSharedTableCatalogCN
    };
    use baad_shared_ffi::shared::ffi::BaadSharedPlatform;

    #[diplomat::opaque]
    pub struct BaadJapanStrategy;
    impl BaadJapanStrategy {
        pub fn build_asset_downloads(
            catalog: &mut BaadSharedBundlePatchPackInfo,
            url: &str,
            platform: BaadSharedPlatform
        ) -> Box<BaadSharedDownloads> {
            let items = baad_native::strategy::JapanStrategy::build_asset_downloads(
                std::mem::take(&mut catalog.0),
                url,
                platform.into()
            );
            Box::new(BaadSharedDownloads(baad_shared::Downloads {
                assets: items,
                tables: Vec::new(),
                media: Vec::new()
            }))
        }

        pub fn build_table_downloads(
            catalog: &mut BaadSharedTableCatalog,
            url: &str
        ) -> Box<BaadSharedDownloads> {
            let items = baad_native::strategy::JapanStrategy::build_table_downloads(
                std::mem::take(&mut catalog.0),
                url
            );
            Box::new(BaadSharedDownloads(baad_shared::Downloads {
                assets: Vec::new(),
                tables: items,
                media: Vec::new()
            }))
        }

        pub fn build_media_downloads(
            catalog: &mut BaadSharedMediaCatalog,
            url: &str,
            platform: BaadSharedPlatform
        ) -> Box<BaadSharedDownloads> {
            let items = baad_native::strategy::JapanStrategy::build_media_downloads(
                std::mem::take(&mut catalog.0),
                url,
                platform.into()
            );
            Box::new(BaadSharedDownloads(baad_shared::Downloads {
                assets: Vec::new(),
                tables: Vec::new(),
                media: items
            }))
        }
    }
    #[diplomat::opaque]
    pub struct BaadChinaStrategy;
    impl BaadChinaStrategy {
        pub fn build_asset_downloads(
            catalog: &mut BaadSharedBundleCatalogCN,
            url: &str,
            platform: BaadSharedPlatform
        ) -> Box<BaadSharedDownloads> {
            let items = baad_native::strategy::ChinaStrategy::build_asset_downloads(
                std::mem::take(&mut catalog.0),
                url,
                platform.into()
            );
            Box::new(BaadSharedDownloads(baad_shared::Downloads {
                assets: items,
                tables: Vec::new(),
                media: Vec::new()
            }))
        }

        pub fn build_table_downloads(
            catalog: &mut BaadSharedTableCatalogCN,
            url: &str
        ) -> Box<BaadSharedDownloads> {
            let items = baad_native::strategy::ChinaStrategy::build_table_downloads(
                std::mem::take(&mut catalog.0),
                url
            );
            Box::new(BaadSharedDownloads(baad_shared::Downloads {
                assets: Vec::new(),
                tables: items,
                media: Vec::new()
            }))
        }

        pub fn build_media_downloads(
            catalog: &mut BaadSharedMediaCatalogCN,
            url: &str
        ) -> Box<BaadSharedDownloads> {
            let items = baad_native::strategy::ChinaStrategy::build_media_downloads(
                std::mem::take(&mut catalog.0),
                url
            );
            Box::new(BaadSharedDownloads(baad_shared::Downloads {
                assets: Vec::new(),
                tables: Vec::new(),
                media: items
            }))
        }
    }
    #[diplomat::opaque]
    pub struct BaadGlobalStrategy;
    impl BaadGlobalStrategy {
        pub fn build_downloads(
            catalog: &mut BaadSharedGlobalCatalogData,
            base_url: &str,
            category: u8
        ) -> Box<BaadSharedDownloads> {
            Box::new(BaadSharedDownloads(baad_native::strategy::GlobalStrategy::build_downloads(
                std::mem::take(&mut catalog.0.resources),
                base_url,
                crate::download_adapter::resource_category(category)
            )))
        }
    }
}
