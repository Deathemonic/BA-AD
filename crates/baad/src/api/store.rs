use std::borrow::Cow;

use baad_shared::{APPLE_LOOKUP_URL, APPSTORE_CODE, GOOGLE_PLAY_URL, MarketConfig, client};
use lazy_regex::regex;
use serde::Deserialize;

use crate::error::CatalogError;

#[derive(Deserialize)]
struct AppleLookup {
    results: Vec<AppleApp>
}

#[derive(Deserialize)]
struct AppleApp {
    version: String
}

/// Reads the published client version from the official store listing. The
/// result is only a hint: it must still be accepted by the Nexon version
/// check.
pub struct StoreClient {
    google_play_url: Cow<'static, str>,
    apple_lookup_url: Cow<'static, str>
}

impl Default for StoreClient {
    fn default() -> Self { Self::new() }
}

impl StoreClient {
    pub const fn new() -> Self {
        Self {
            google_play_url: Cow::Borrowed(GOOGLE_PLAY_URL),
            apple_lookup_url: Cow::Borrowed(APPLE_LOOKUP_URL)
        }
    }

    /// Uses different store endpoints, e.g. a local test server. The market
    /// game ID is appended to each URL.
    pub fn with_urls(
        google_play_url: impl Into<String>,
        apple_lookup_url: impl Into<String>
    ) -> Self {
        Self {
            google_play_url: Cow::Owned(google_play_url.into()),
            apple_lookup_url: Cow::Owned(apple_lookup_url.into())
        }
    }

    pub async fn latest_version(&self, market: &MarketConfig) -> Result<String, CatalogError> {
        let apple = market.market_code == APPSTORE_CODE;
        let base = if apple { &self.apple_lookup_url } else { &self.google_play_url };
        let url = format!("{base}{}", market.market_game_id);
        let response = client().get(&url).send().await?.error_for_status()?;
        let body = response.text().await?;

        let version = if apple {
            serde_json::from_str::<AppleLookup>(&body)
                .ok()
                .and_then(|lookup| lookup.results.into_iter().next())
                .map(|app| app.version)
        } else {
            google_play_version(&body).map(String::from)
        };
        version.filter(|version| is_version(version)).ok_or_else(|| {
            CatalogError::VersionDiscovery {
                reason: format!("No client version found at {url}").into()
            }
        })
    }
}

/// The details page embeds the current version as `[[["1.93.454564"]]`. Any
/// other version-like text on the page (release notes, scripts) is ignored.
fn google_play_version(page: &str) -> Option<&str> {
    regex!(r#"\[\[\["(\d+\.\d+\.\d+)"\]\]"#)
        .captures(page)
        .and_then(|captures| captures.get(1))
        .map(|version| version.as_str())
}

pub(crate) fn is_version(version: &str) -> bool {
    let parts: Vec<_> = version.split('.').collect();
    parts.len() == 3
        && parts.iter().all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn google_play_version_ignores_other_version_like_text() {
        let page = r#"<script>var v="4.5.6";</script>[[["1.93.454564"]],[[["30"]]]"#;
        assert_eq!(google_play_version(page), Some("1.93.454564"));
        assert_eq!(google_play_version("Requires Android 7.1.2 and up"), None);
    }

    #[test]
    fn versions_must_have_three_numeric_parts() {
        assert!(is_version("1.93.454564"));
        for version in ["", "1.93", "1.93.x", "1..2", "1.2.3.4", "1.2.3-beta"] {
            assert!(!is_version(version), "{version}");
        }
    }
}
