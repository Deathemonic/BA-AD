use baad_dm::client;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::reqwest::Url;

pub fn parse_url(url: &str) -> Result<Url, baad_dm::Error> {
    Url::parse(url).map_err(|error| baad_dm::Error::InvalidUrl {
        url: url.into(),
        reason: error.to_string().into()
    })
}

pub fn create_proxy(proxy_url: Option<&str>) -> Result<Option<reqwest::Proxy>, baad_dm::Error> {
    proxy_url
        .map(|url| {
            reqwest::Proxy::all(url).map_err(|error| baad_dm::Error::InvalidUrl {
                url: url.into(),
                reason: error.to_string().into()
            })
        })
        .transpose()
}

pub fn default_client() -> Result<ClientWithMiddleware, baad_dm::Error> {
    baad_dm::create_http_client(baad_dm::HttpClientConfig::builder().build()).map_err(Into::into)
}

pub fn validate_limits(
    concurrency: usize,
    chunks: usize,
    concurrent_chunks: usize
) -> Result<(), String> {
    if concurrency == 0 || chunks == 0 || concurrent_chunks == 0 {
        return Err("Download concurrency and chunk limits must be positive".into());
    }
    Ok(())
}

pub async fn resolve_url(url: &str) -> Result<String, baad_dm::Error> {
    client::resolve_url(&default_client()?, url).await
}

pub async fn zip_index(url: &str) -> Result<baad_dm::ZipIndex, baad_dm::Error> {
    let client = default_client()?;
    let url = parse_url(url)?;
    baad_dm::ZipExtractor::new(&client, &url).await?.build_index().await
}

pub async fn zip_extract_file(url: &str, target: &str) -> Result<bytes::Bytes, baad_dm::Error> {
    let client = default_client()?;
    let url = parse_url(url)?;
    baad_dm::ZipExtractor::new(&client, &url).await?.extract_file(target).await
}
