use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::reqwest::Url;
use reqwest_middleware::reqwest::header::ACCEPT_ENCODING;
use tokio::sync::Semaphore;

use crate::client::{parse_accept_ranges, parse_content_length};
use crate::error::Error;

#[derive(Debug)]
pub struct ServerInfo {
    pub resumable: bool,
    pub content_length: Option<u64>,
    pub resolved_url: String
}

pub async fn inspect(
    client: &ClientWithMiddleware,
    requests: &Semaphore,
    url: &Url
) -> Result<ServerInfo, Error> {
    let _permit = requests.acquire().await.map_err(|_| Error::LimiterClosed)?;

    let response = client.head(url.clone()).header(ACCEPT_ENCODING, "identity").send().await?;
    response.error_for_status_ref()?;

    Ok(ServerInfo {
        resumable: parse_accept_ranges(response.headers()),
        content_length: parse_content_length(response.headers()),
        resolved_url: response.url().to_string()
    })
}
