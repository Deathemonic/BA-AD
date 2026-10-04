#[diplomat::bridge]
pub mod ffi {
    use baad_shared_ffi::error::ffi::BaadError;
    use baad_shared_ffi::models::ffi::{
        BaadSharedChinaState,
        BaadSharedDomain,
        BaadSharedGameBaseConfig,
        BaadSharedGameFile,
        BaadSharedGameJsonConfig,
        BaadSharedGameJsonData,
        BaadSharedGlobalAddressable,
        BaadSharedGlobalCatalogData
    };
    use baad_shared_ffi::shared::ffi::BaadSharedMarketConfig;

    #[diplomat::opaque]
    pub struct BaadNexonClient;

    impl BaadNexonClient {
        pub fn get_catalog(url: &str) -> Result<Box<BaadSharedGlobalCatalogData>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_native::api::NexonClient::new().get_catalog(url))
                .map(|value| Box::new(BaadSharedGlobalCatalogData(value)))
        }

        pub fn get_addressable(
            config: &BaadSharedMarketConfig,
            version: &str,
            build_number: &str
        ) -> Result<Box<BaadSharedGlobalAddressable>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_native::api::NexonClient::new().get_addressable(
                &config.0,
                version,
                build_number
            ))
            .map(|value| Box::new(BaadSharedGlobalAddressable(value)))
        }
    }

    #[diplomat::opaque]
    pub struct BaadYoStarClient;

    impl BaadYoStarClient {
        pub fn get_base_config() -> Result<Box<BaadSharedGameBaseConfig>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(
                baad_native::api::YoStarClient::new().get_base_config()
            )
            .map(|value| Box::new(BaadSharedGameBaseConfig(value)))
        }

        pub fn get_domain() -> Result<Box<BaadSharedDomain>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_native::api::YoStarClient::new().get_domain())
                .map(|value| Box::new(BaadSharedDomain(value)))
        }

        pub fn get_json_config(
            version: &str,
            path: &str
        ) -> Result<Box<BaadSharedGameJsonConfig>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(
                baad_native::api::YoStarClient::new().get_json_config(version, path)
            )
            .map(|value| Box::new(BaadSharedGameJsonConfig(value)))
        }

        pub fn get_json_data(url: &str) -> Result<Box<BaadSharedGameJsonData>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(
                baad_native::api::YoStarClient::new().get_json_data(url)
            )
            .map(|value| Box::new(BaadSharedGameJsonData(value)))
        }

        pub fn get_resources_asset() -> Result<Box<BaadResourcesAsset>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(
                baad_native::api::YoStarClient::new().get_resources_asset()
            )
            .map(|value| Box::new(BaadResourcesAsset(value)))
        }
    }

    #[diplomat::opaque]
    pub struct BaadResourcesAsset(pub (String, baad_shared::GameFile));

    impl BaadResourcesAsset {
        pub fn url<'a>(&'a self) -> &'a str { &self.0.0 }

        pub const fn file<'a>(&'a self) -> &'a BaadSharedGameFile {
            BaadSharedGameFile::borrow(&self.0.1)
        }
    }

    #[diplomat::opaque]
    pub struct BaadRoStarClient;

    impl BaadRoStarClient {
        pub fn get_state(version: &str) -> Result<Box<BaadSharedChinaState>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(
                baad_native::api::RoStarClient::new().get_state(version)
            )
            .map(|value| Box::new(BaadSharedChinaState(value)))
        }
    }
}
