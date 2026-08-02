use std::path::{Path, PathBuf};
use std::sync::Arc;

use reqwest_middleware::ClientWithMiddleware;
use tokio::fs;

use crate::client::{parse_accept_ranges, parse_content_length};
use crate::download::Download;
use crate::error::Error;
use crate::zip::ZipCache;

#[derive(Debug)]
pub struct FetchCtx<'a> {
    pub client: Arc<ClientWithMiddleware>,
    pub download: &'a Download,
    pub file_path: PathBuf,
    pub cache: &'a ZipCache
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

    let resumable = parse_accept_ranges(headers);
    let content_length = parse_content_length(headers);

    Ok((resumable, content_length, resolved_url))
}

pub async fn ensure_parent_dir(path: &Path) -> Result<(), Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    Ok(())
}
