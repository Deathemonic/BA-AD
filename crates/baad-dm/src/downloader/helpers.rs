use std::path::{Path, PathBuf};

use reqwest_middleware::ClientWithMiddleware;
use tokio::fs;

use crate::download::Download;
use crate::error::Error;

#[derive(Debug)]
pub struct FetchCtx<'a> {
    pub client: &'a ClientWithMiddleware,
    pub download: &'a Download,
    pub file_path: PathBuf
}

pub struct StreamOpts {
    pub size_on_disk: u64,
    pub resumable: bool
}

pub async fn check_server(
    client: &ClientWithMiddleware,
    download: &Download
) -> Result<(bool, Option<u64>, String), Error> {
    let res = client.head(download.url.clone()).send().await?;

    let resolved_url = res.url().to_string();
    let headers = res.headers();

    let resumable =
        headers.get("accept-ranges").and_then(|v| v.to_str().ok()).is_some_and(|v| v != "none");

    let content_length =
        headers.get("content-length").and_then(|v| v.to_str().ok()).and_then(|v| v.parse().ok());

    Ok((resumable, content_length, resolved_url))
}

pub async fn ensure_parent_dir(path: &Path) -> Result<(), Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    Ok(())
}
