#[diplomat::bridge]
pub mod ffi {
    use std::fmt::Write;

    use crate::adapter::init_client;
    use crate::error::error;
    use crate::error::ffi::BaadError;

    pub enum BaadSharedPlatform {}

    pub enum BaadSharedBuildType {}

    impl BaadSharedPlatform {
        pub fn patch_pack(self) -> &'static str { baad_shared::Platform::from(self).patch_pack() }

        pub fn media_path(self) -> &'static str { baad_shared::Platform::from(self).media_path() }

        pub fn display_name(self) -> &'static str {
            baad_shared::Platform::from(self).display_name()
        }

        pub fn ensure_supported(self) -> Result<(), Box<BaadError>> {
            baad_shared::Platform::from(self).ensure_supported().map_err(error)
        }
    }

    #[diplomat::opaque]
    pub struct BaadSharedMarketConfig(pub baad_shared::MarketConfig);

    impl BaadSharedMarketConfig {
        pub fn for_global(
            platform: BaadSharedPlatform,
            build_type: BaadSharedBuildType
        ) -> Result<Box<Self>, Box<BaadError>> {
            baad_shared::MarketConfig::for_global(platform.into(), build_type.into())
                .map(|config| Box::new(Self(config)))
                .map_err(error)
        }

        pub const fn market_game_id(&self) -> &'static str { self.0.market_game_id }

        pub const fn market_code(&self) -> &'static str { self.0.market_code }
    }

    #[diplomat::opaque]
    pub struct BaadSharedClient;

    impl BaadSharedClient {
        pub fn init(proxy: &str, user_agent: &str, no_proxy: bool) -> Result<(), Box<BaadError>> {
            init_client(proxy, user_agent, no_proxy).map_err(error)
        }
    }

    #[diplomat::opaque]
    pub struct BaadSharedVersion;

    impl BaadSharedVersion {
        pub fn extract<'a>(text: &'a str) -> Option<&'a str> {
            baad_shared::REGEX_VERSION.find(text).map(|value| value.as_str())
        }
    }

    pub enum BaadSharedChinaMediaType {}

    impl BaadSharedChinaMediaType {
        pub fn from_value(value: i32) -> Option<Self> {
            baad_shared::ChinaMediaType::from_repr(value).map(Into::into)
        }

        pub fn extension(self, output: &mut DiplomatWrite) {
            let value = baad_shared::ChinaMediaType::from(self);
            let _ = output.write_str(value.as_ref());
        }
    }
}
