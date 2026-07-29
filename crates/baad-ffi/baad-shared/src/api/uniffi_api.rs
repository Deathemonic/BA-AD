//! UniFFI bindings for `baad-shared`. `init_client` builds the `reqwest`
//! client from options since a client cannot cross FFI; constant getters and
//! mirrors are generated into `shadow.rs`.

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum ClientError {
    #[error("Invalid proxy URL: {reason}")]
    InvalidProxy { reason: String },

    #[error("Failed to build HTTP client: {reason}")]
    BuildFailed { reason: String }
}

#[uniffi::export]
pub fn platform_patch_pack(platform: baad_shared::Platform) -> String {
    String::from(platform.patch_pack())
}

#[uniffi::export]
pub fn platform_media_path(platform: baad_shared::Platform) -> String {
    String::from(platform.media_path())
}

#[uniffi::export]
pub fn platform_display_name(platform: baad_shared::Platform) -> String {
    String::from(platform.display_name())
}

#[uniffi::export]
pub fn platform_ensure_supported(
    platform: baad_shared::Platform
) -> Result<(), baad_shared::ServerConfigError> {
    platform.ensure_supported()
}

#[uniffi::export]
pub fn china_media_type_from_repr(value: i32) -> Option<baad_shared::ChinaMediaType> {
    baad_shared::ChinaMediaType::from_repr(value)
}

#[uniffi::export]
pub fn hash_value_as_string(hash: baad_shared::HashValue) -> String { hash.as_string() }

#[derive(uniffi::Object)]
pub struct MarketConfig {
    inner: baad_shared::MarketConfig
}

#[uniffi::export]
impl MarketConfig {
    #[uniffi::constructor]
    pub fn for_global(
        platform: baad_shared::Platform,
        build_type: baad_shared::BuildType
    ) -> Result<Self, baad_shared::ServerConfigError> {
        Ok(MarketConfig {
            inner: baad_shared::MarketConfig::for_global(platform, build_type)?
        })
    }

    pub fn market_game_id(&self) -> String { String::from(self.inner.market_game_id) }

    pub fn market_code(&self) -> String { String::from(self.inner.market_code) }
}

impl MarketConfig {
    #[allow(dead_code)]
    pub(crate) fn native(&self) -> baad_shared::MarketConfig {
        baad_shared::MarketConfig {
            market_game_id: self.inner.market_game_id,
            market_code: self.inner.market_code
        }
    }
}

#[derive(Debug, Clone, Default, uniffi::Record)]
pub struct ClientConfig {
    pub proxy: Option<String>,
    pub user_agent: Option<String>
}

#[uniffi::export]
pub fn init_client(config: ClientConfig) -> Result<(), ClientError> {
    let mut builder = reqwest::Client::builder();

    if let Some(proxy_url) = &config.proxy {
        let proxy = reqwest::Proxy::all(proxy_url)
            .map_err(|error| ClientError::InvalidProxy { reason: error.to_string() })?;
        builder = builder.proxy(proxy);
    }

    if let Some(user_agent) = &config.user_agent {
        builder = builder.user_agent(user_agent);
    }

    let client =
        builder.build().map_err(|error| ClientError::BuildFailed { reason: error.to_string() })?;

    baad_shared::set_client(client);
    Ok(())
}

#[uniffi::export]
pub fn extract_version(text: &str) -> Option<String> {
    baad_shared::REGEX_VERSION.find(text).map(|found| String::from(found.as_str()))
}

include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
