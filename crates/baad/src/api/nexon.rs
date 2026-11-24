use std::borrow::Cow;

use baad_shared::{GLOBAL_API_URL, GlobalAddressable, GlobalCatalog, MarketConfig, client};
use serde::Deserialize;

use crate::api::store::is_version;
use crate::error::CatalogError;

#[derive(Deserialize)]
struct ErrorResponse {
    error: ErrorBody
}

#[derive(Deserialize)]
struct ErrorBody {
    code: i64,
    message: String
}

pub struct NexonClient {
    api_url: Cow<'static, str>
}

impl Default for NexonClient {
    fn default() -> Self { Self::new() }
}

impl NexonClient {
    pub const fn new() -> Self {
        Self {
            api_url: Cow::Borrowed(GLOBAL_API_URL)
        }
    }

    /// Uses a different version-check endpoint, e.g. a local test server.
    pub fn with_api_url(api_url: impl Into<String>) -> Self {
        Self {
            api_url: Cow::Owned(api_url.into())
        }
    }

    /// Asks the patch server which patch belongs to a client build. Nexon only
    /// answers for builds it has registered; others are rejected with an error
    /// code.
    pub async fn get_addressable(
        &self,
        market_config: &MarketConfig,
        version: &str,
        build_number: &str
    ) -> Result<GlobalAddressable, CatalogError> {
        let response = client()
            .post(self.api_url.as_ref())
            .json(&serde_json::json!({
                "market_game_id": market_config.market_game_id,
                "market_code": market_config.market_code,
                "curr_build_version": version,
                "curr_build_number": build_number
            }))
            .send()
            .await?;

        let status = response.status();
        let body = response.bytes().await?;
        if let Ok(ErrorResponse { error }) = serde_json::from_slice(&body) {
            return Err(CatalogError::VersionCheck {
                version: version.into(),
                code: error.code,
                message: error.message.into()
            });
        }
        if !status.is_success() {
            return Err(CatalogError::VersionCheck {
                version: version.into(),
                code: status.as_u16().into(),
                message: "Unexpected HTTP status from the version check".into()
            });
        }

        let addressable: GlobalAddressable = serde_json::from_slice(&body)?;
        if !is_version(&addressable.latest_build_version)
            || addressable.patch.resource_path.is_empty()
        {
            return Err(CatalogError::VersionDiscovery {
                reason: "Version check response has no build version or resource path".into()
            });
        }
        Ok(addressable)
    }

    pub async fn get_catalog(&self, resource_path: &str) -> Result<GlobalCatalog, CatalogError> {
        let response = client().get(resource_path).send().await?.error_for_status()?;
        let catalog = response.json().await?;
        Ok(catalog)
    }
}
