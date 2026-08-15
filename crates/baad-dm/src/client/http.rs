use std::time::Duration;

use reqwest::header::HeaderMap;
use reqwest_middleware::reqwest::{Client, Error as ReqwestError, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;

use crate::client::config::HttpClientConfig;
use crate::error::Error;

pub fn create_http_client(config: HttpClientConfig) -> Result<ClientWithMiddleware, ReqwestError> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(config.retries);

    let mut builder = Client::builder()
        .pool_max_idle_per_host(config.pool_max_idle)
        .pool_idle_timeout(Duration::from_secs(90))
        .tcp_nodelay(config.tcp_nodelay)
        .connect_timeout(Duration::from_secs(30))
        .timeout(Duration::from_mins(5));

    if let Some(proxy) = config.proxy {
        builder = builder.proxy(proxy);
    }

    if let Some(headers) = config.headers {
        builder = builder.default_headers(headers);
    }

    Ok(ClientBuilder::new(builder.build()?)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build())
}

pub fn get_content_length(response: &Response) -> Option<u64> {
    response.headers().get("content-range").map_or_else(
        || response.content_length(),
        |content_range| {
            content_range
                .to_str()
                .ok()
                .and_then(|range| range.split('/').next_back())
                .and_then(|size| size.trim().parse::<u64>().ok())
        }
    )
}

pub fn create_range_header(start: u64, end: Option<u64>) -> String {
    end.map_or_else(|| format!("bytes={start}-"), |e| format!("bytes={start}-{e}"))
}

pub fn parse_accept_ranges(headers: &HeaderMap) -> bool {
    headers.get("accept-ranges").and_then(|v| v.to_str().ok()).is_some_and(|v| v != "none")
}

pub fn parse_content_length(headers: &HeaderMap) -> Option<u64> {
    headers.get("content-length").and_then(|v| v.to_str().ok()).and_then(|v| v.parse().ok())
}

pub async fn resolve_url(client: &ClientWithMiddleware, url: &str) -> Result<String, Error> {
    let res = client.head(url).send().await?;

    Ok(res.url().to_string())
}
