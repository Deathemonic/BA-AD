use super::config::HttpClientConfig;

use std::time::Duration;

use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

pub fn create_http_client(config: HttpClientConfig) -> Result<ClientWithMiddleware, reqwest_middleware::reqwest::Error> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(config.retries);

    let mut builder = reqwest_middleware::reqwest::Client::builder()
        .pool_max_idle_per_host(config.pool_max_idle)
        .pool_idle_timeout(Duration::from_secs(90))
        .tcp_nodelay(config.tcp_nodelay)
        .connect_timeout(Duration::from_secs(30))
        .timeout(Duration::from_secs(300));

    if let Some(proxy) = config.proxy {
        builder = builder.proxy(proxy);
    }

    if let Some(headers) = config.headers {
        builder = builder.default_headers(headers);
    }

    let inner_client = builder.build()?;

    let client = ClientBuilder::new(inner_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    Ok(client)
}

pub fn get_content_length(response: &reqwest_middleware::reqwest::Response) -> Option<u64> {
    if let Some(content_range) = response.headers().get("content-range") {
        content_range
            .to_str()
            .ok()
            .and_then(|range| range.split('/').next_back())
            .and_then(|size| size.trim().parse::<u64>().ok())
    } else {
        response.content_length()
    }
}

pub async fn resolve_url(client: &ClientWithMiddleware, url: &str) -> Result<String, String> {
    let res = client
        .head(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(res.url().to_string())
}
