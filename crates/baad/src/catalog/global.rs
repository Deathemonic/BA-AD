use std::path::PathBuf;

use baad_shared::platform::{BuildType, MarketConfig, Platform};
use baad_shared::{
    API_FILENAME,
    ApiData,
    Downloads,
    GlobalAddressable,
    GlobalCatalog as GlobalCatalogData
};
use baad_utils::file::get_data_path;
use baad_utils::json::{load, update};
use tracing::{info, warn};

use crate::api::{NexonClient, StoreClient};
use crate::catalog::traits::Catalog;
use crate::cdn::GlobalCdn;
use crate::download::ResourceCategory;
use crate::error::CatalogError;
use crate::strategy::GlobalStrategy;

struct GlobalPaths {
    api: PathBuf
}

pub struct GlobalCatalog {
    nexon_client: NexonClient,
    store_client: StoreClient,
    client_version: Option<String>,
    category: ResourceCategory,
    platform: Platform,
    build_type: BuildType,
    paths: GlobalPaths
}

impl GlobalCatalog {
    pub fn new(
        category: ResourceCategory,
        platform: Platform,
        build_type: BuildType
    ) -> Result<Self, CatalogError> {
        MarketConfig::for_global(platform, build_type)?;

        Ok(Self {
            nexon_client: NexonClient::new(),
            store_client: StoreClient::new(),
            client_version: None,
            category,
            platform,
            build_type,
            paths: GlobalPaths {
                api: get_data_path(API_FILENAME)?
            }
        })
    }

    /// Uses this client build instead of discovering the latest one, e.g. to
    /// reproduce a snapshot. Nexon must still accept the version.
    #[must_use]
    pub fn with_client_version(mut self, version: impl Into<String>) -> Self {
        self.client_version = Some(version.into());
        self
    }

    /// Replaces the version-check and store endpoints, e.g. for tests.
    #[must_use]
    pub fn with_clients(mut self, nexon_client: NexonClient, store_client: StoreClient) -> Self {
        self.nexon_client = nexon_client;
        self.store_client = store_client;
        self
    }

    /// Finds a client build Nexon accepts and returns its patch information.
    ///
    /// Nexon rejects unregistered builds, so a candidate is needed: the store
    /// listing first, then the version cached by a previous run. Its reply
    /// names the latest build, which is queried again so the patch matches it.
    pub async fn discover_addressable(
        &self,
        cached_version: Option<&str>
    ) -> Result<GlobalAddressable, CatalogError> {
        let market_config = MarketConfig::for_global(self.platform, self.build_type)?;
        if let Some(version) = &self.client_version {
            return self.addressable(&market_config, version).await;
        }

        let mut errors = Vec::new();
        let mut candidates = Vec::new();
        match self.store_client.latest_version(&market_config).await {
            Ok(version) => candidates.push(version),
            Err(error) => {
                warn!(cause = %error, "Store version lookup failed");
                errors.push(format!("store lookup: {error}"));
            }
        }
        if let Some(version) =
            cached_version.filter(|version| !candidates.iter().any(|c| c == version))
        {
            candidates.push(version.into());
        }

        for candidate in candidates {
            match self.addressable(&market_config, &candidate).await {
                Ok(addressable) if addressable.latest_build_version == candidate => {
                    return Ok(addressable);
                }
                Ok(addressable) => {
                    info!(from = %candidate, to = %addressable.latest_build_version, "Newer client build");
                    return self
                        .addressable(&market_config, &addressable.latest_build_version)
                        .await;
                }
                Err(error) => errors.push(format!("{candidate}: {error}"))
            }
        }

        Err(CatalogError::VersionDiscovery {
            reason: format!(
                "no accepted client build ({}); pass the client version explicitly",
                if errors.is_empty() { "no candidates".into() } else { errors.join("; ") }
            )
            .into()
        })
    }

    async fn addressable(
        &self,
        market_config: &MarketConfig,
        version: &str
    ) -> Result<GlobalAddressable, CatalogError> {
        let build_number = version.rsplit('.').next().ok_or(CatalogError::DeserializationFailed)?;
        self.nexon_client.get_addressable(market_config, version, build_number).await
    }

    async fn full_update(&self, version: String) -> Result<String, CatalogError> {
        let market_config = MarketConfig::for_global(self.platform, self.build_type)?;

        let build_number =
            version.split('.').next_back().ok_or(CatalogError::DeserializationFailed)?;

        let addressable =
            self.nexon_client.get_addressable(&market_config, &version, build_number).await?;

        let catalog_url = addressable.patch.resource_path;
        let platform: &'static str = self.platform.into();
        let build_type: &'static str = self.build_type.into();

        update(&self.paths.api, |data: &mut ApiData| {
            data.global.version = version;
            data.global.catalog_url.clone_from(&catalog_url);
            data.global.platform = platform.into();
            data.global.build_type = build_type.into();
        })
        .await?;

        Ok(catalog_url)
    }

    pub async fn get_catalog_url(&self, version: &str) -> Result<(String, bool), CatalogError> {
        let platform: &'static str = self.platform.into();
        let build_type: &'static str = self.build_type.into();

        let api_data = load::<ApiData>(&self.paths.api)
            .await
            .ok()
            .filter(|d| {
                d.global.version == version
                    && d.global.platform == platform
                    && d.global.build_type == build_type
            })
            .map(|d| d.global.catalog_url)
            .filter(|url| !url.is_empty());

        if let Some(catalog_url) = api_data {
            return Ok((catalog_url, true));
        }

        info!("Fetching latest update");
        let catalog_url = self.full_update(version.into()).await?;
        Ok((catalog_url, false))
    }

    pub async fn fetch_catalogs(
        &self,
        catalog_url: &str
    ) -> Result<GlobalCatalogData, CatalogError> {
        let cdn = GlobalCdn::new(catalog_url.into(), self.platform);
        let catalog = cdn.fetch().await?;
        Ok(catalog)
    }
}

impl Catalog for GlobalCatalog {
    type Resources = GlobalCatalogData;

    async fn fetch_resources(&self) -> Result<(String, Self::Resources, bool), CatalogError> {
        let platform: &'static str = self.platform.into();
        let build_type: &'static str = self.build_type.into();
        let cached = load::<ApiData>(&self.paths.api)
            .await
            .ok()
            .filter(|d| d.global.platform == platform && d.global.build_type == build_type);

        let addressable =
            self.discover_addressable(cached.as_ref().map(|d| d.global.version.as_str())).await?;
        let version = self.client_version.clone().unwrap_or(addressable.latest_build_version);
        let catalog_url = addressable.patch.resource_path;
        info!(version = %version, patch = addressable.patch.patch_version, "Version");

        // The patch can change without a new client build, so compare the
        // catalog URL as well as the version.
        let up_to_date = cached
            .is_some_and(|d| d.global.version == version && d.global.catalog_url == catalog_url);
        if !up_to_date {
            update(&self.paths.api, |data: &mut ApiData| {
                data.global.version.clone_from(&version);
                data.global.catalog_url.clone_from(&catalog_url);
                data.global.platform = platform.into();
                data.global.build_type = build_type.into();
            })
            .await?;
        }

        let base_url = GlobalCdn::derive_base_url(&catalog_url)
            .ok_or(CatalogError::DeserializationFailed)?
            .into();

        let catalog = self.fetch_catalogs(&catalog_url).await?;
        Ok((base_url, catalog, up_to_date))
    }

    fn build_downloads(&self, resources: Self::Resources, url: &str) -> Downloads {
        GlobalStrategy::build_downloads(resources.resources, url, self.category)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use baad_shared::platform::{BuildType, Platform};
    use serde_json::{Value, json};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::task::JoinHandle;

    use super::GlobalCatalog;
    use crate::api::{NexonClient, StoreClient};
    use crate::download::ResourceCategory;
    use crate::error::CatalogError;

    type Handler = dyn Fn(&str, &str) -> (u16, String) + Send + Sync;

    /// Serves `handler(path, body)` and records each `path body` it saw.
    struct Fixture {
        base_url: String,
        requests: Arc<Mutex<Vec<String>>>,
        task: JoinHandle<()>
    }

    impl Fixture {
        async fn new(
            handler: impl Fn(&str, &str) -> (u16, String) + Send + Sync + 'static
        ) -> Self {
            let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind fixture");
            let base_url = format!("http://{}", listener.local_addr().expect("fixture address"));
            let requests = Arc::new(Mutex::new(Vec::new()));
            let log = Arc::clone(&requests);
            let handler: Arc<Handler> = Arc::new(handler);
            let task = tokio::spawn(async move {
                while let Ok((mut stream, _)) = listener.accept().await {
                    let mut request = Vec::new();
                    let mut buffer = [0; 4096];
                    let (head_end, length) = loop {
                        let count = stream.read(&mut buffer).await.expect("read request");
                        if count == 0 {
                            return;
                        }
                        request.extend_from_slice(&buffer[..count]);
                        if let Some(end) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                            let head = String::from_utf8_lossy(&request[..end]).to_lowercase();
                            let length = head
                                .lines()
                                .find_map(|line| line.strip_prefix("content-length:"))
                                .map_or(0, |value| value.trim().parse().expect("length"));
                            break (end + 4, length);
                        }
                    };
                    while request.len() < head_end + length {
                        let count = stream.read(&mut buffer).await.expect("read body");
                        request.extend_from_slice(&buffer[..count]);
                    }
                    let head = String::from_utf8_lossy(&request[..head_end]).into_owned();
                    let path = head.split_whitespace().nth(1).unwrap_or_default().to_owned();
                    let body = String::from_utf8_lossy(&request[head_end..]).into_owned();
                    log.lock().expect("request log").push(format!("{path} {body}"));
                    let (status, response) = handler(&path, &body);
                    let reply = format!(
                        "HTTP/1.1 {status} Fixture\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response}",
                        response.len()
                    );
                    let _ = stream.write_all(reply.as_bytes()).await;
                }
            });
            Self { base_url, requests, task }
        }

        fn catalog(&self, platform: Platform) -> GlobalCatalog {
            GlobalCatalog::new(ResourceCategory::ALL, platform, BuildType::Standard)
                .expect("Global catalog")
                .with_clients(
                    NexonClient::with_api_url(format!("{}/version-check", self.base_url)),
                    StoreClient::with_urls(
                        format!("{}/play?id=", self.base_url),
                        format!("{}/lookup?id=", self.base_url)
                    )
                )
        }

        fn requested_versions(&self) -> Vec<String> {
            self.requests
                .lock()
                .expect("request log")
                .iter()
                .filter_map(|line| line.split_once(' '))
                .filter(|(path, _)| *path == "/version-check")
                .filter_map(|(_, body)| serde_json::from_str::<Value>(body).ok())
                .filter_map(|body| body["curr_build_version"].as_str().map(String::from))
                .collect()
        }

        fn store_requests(&self) -> usize {
            let requests = self.requests.lock().expect("request log");
            requests.iter().filter(|line| !line.starts_with("/version-check")).count()
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) { self.task.abort(); }
    }

    /// Nexon's reply for a registered build: latest is 1.93.454564, and each
    /// build gets its own patch so tests can tell which query answered.
    fn version_check(body: &str, registered: &[&str]) -> (u16, String) {
        let request: Value = serde_json::from_str(body).expect("version check body");
        let version = request["curr_build_version"].as_str().unwrap_or_default();
        if !registered.contains(&version) {
            let error = json!({"api_version": "v1.1", "error": {"transient": false,
                "type": "PatchVersionCheckException", "code": 90104,
                "message": "Unregistered client current build info."}});
            return (400, error.to_string());
        }
        let reply = json!({"api_version": "v1.1", "market_game_id": "com.nexon.bluearchive",
            "latest_build_version": "1.93.454564", "latest_build_number": "454564",
            "min_build_version": "1.93.452511", "min_build_number": "452511",
            "patch": {"patch_version": 1750, "bdiff_path": [],
                "resource_path": format!("https://cdn.example/{version}/resource-data.json")}});
        (200, reply.to_string())
    }

    const REGISTERED: &[&str] = &["1.93.452511", "1.93.454564"];
    const PLAY_PAGE: &str = r#"<html>Android 7.1.2 [[["1.93.454564"]]</html>"#;

    #[tokio::test]
    async fn store_version_confirmed_by_nexon() -> Result<(), CatalogError> {
        let fixture = Fixture::new(|path, body| match path {
            "/version-check" => version_check(body, REGISTERED),
            _ => (200, PLAY_PAGE.into())
        })
        .await;
        let addressable = fixture.catalog(Platform::Android).discover_addressable(None).await?;
        assert_eq!(
            addressable.patch.resource_path,
            "https://cdn.example/1.93.454564/resource-data.json"
        );
        assert_eq!(fixture.requested_versions(), ["1.93.454564"]);
        Ok(())
    }

    #[tokio::test]
    async fn failed_store_falls_back_to_cached_build_and_follows_latest() -> Result<(), CatalogError>
    {
        // The error page contains a version-like string that must be ignored.
        let fixture = Fixture::new(|path, body| match path {
            "/version-check" => version_check(body, REGISTERED),
            _ => (503, "Service 1.2.3 unavailable".into())
        })
        .await;
        let addressable =
            fixture.catalog(Platform::Android).discover_addressable(Some("1.93.452511")).await?;
        assert_eq!(
            addressable.patch.resource_path,
            "https://cdn.example/1.93.454564/resource-data.json"
        );
        assert_eq!(fixture.requested_versions(), ["1.93.452511", "1.93.454564"]);
        Ok(())
    }

    #[tokio::test]
    async fn unregistered_candidates_report_the_nexon_error() {
        let fixture = Fixture::new(|path, body| match path {
            "/version-check" => version_check(body, REGISTERED),
            _ => (200, "<html>no version here</html>".into())
        })
        .await;
        let Err(error) =
            fixture.catalog(Platform::Android).discover_addressable(Some("1.92.0")).await
        else {
            panic!("No registered candidate");
        };
        let error = error.to_string();
        assert!(error.contains("90104") && error.contains("1.92.0"), "{error}");
        assert!(error.contains("No client version found"), "{error}");
        assert!(error.contains("client version explicitly"), "{error}");
    }

    #[tokio::test]
    async fn explicit_client_version_skips_store_lookup() -> Result<(), CatalogError> {
        let fixture = Fixture::new(|path, body| match path {
            "/version-check" => version_check(body, REGISTERED),
            _ => panic!("store must not be queried")
        })
        .await;
        let addressable = fixture
            .catalog(Platform::Android)
            .with_client_version("1.93.452511")
            .discover_addressable(Some("1.93.454564"))
            .await?;
        assert_eq!(
            addressable.patch.resource_path,
            "https://cdn.example/1.93.452511/resource-data.json"
        );
        assert_eq!(fixture.requested_versions(), ["1.93.452511"]);
        assert_eq!(fixture.store_requests(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn ios_uses_the_apple_lookup() -> Result<(), CatalogError> {
        let fixture = Fixture::new(|path, body| match path {
            "/version-check" => version_check(body, &["1.93.452024", "1.93.454564"]),
            "/lookup?id=1571873795" => (
                200,
                json!({"resultCount": 1,
                "results": [{"version": "1.93.452024"}]})
                .to_string()
            ),
            _ => (404, String::new())
        })
        .await;
        fixture.catalog(Platform::Ios).discover_addressable(None).await?;
        assert_eq!(fixture.requested_versions(), ["1.93.452024", "1.93.454564"]);
        Ok(())
    }

    #[tokio::test]
    async fn malformed_version_check_responses_are_errors() {
        for (status, body) in [(200, "<html>1.93.454564</html>"), (502, "Bad gateway")] {
            let fixture = Fixture::new(move |path, _| match path {
                "/version-check" => (status, body.into()),
                _ => (200, PLAY_PAGE.into())
            })
            .await;
            let result = fixture.catalog(Platform::Android).discover_addressable(None).await;
            let Err(error) = result else { panic!("{status} {body} was accepted") };
            assert!(matches!(error, CatalogError::VersionDiscovery { .. }), "{error:?}");
        }
    }
}
