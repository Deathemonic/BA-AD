use std::path::Path;
use std::sync::Arc;

use reqwest_middleware::reqwest::Url;

use crate::client::ServerInfo;
use crate::downloader::DownloaderConfig;
use crate::execution::{Cancellation, ProgressTracker, SharedExecution};

pub const WRITE_BUFFER_SIZE: usize = 256 * 1024;

#[derive(Debug)]
pub struct TransferRequest<'a> {
    pub url: &'a Url,
    pub path: &'a Path,
    pub expected_size: Option<u64>,
    pub expected_hash: Option<&'a String>,
    pub progress_index: usize
}

pub struct TransferContext<'a> {
    pub shared: &'a SharedExecution,
    pub config: &'a DownloaderConfig<'a>,
    pub progress: Arc<ProgressTracker>,
    pub cancellation: Arc<Cancellation>
}

pub struct Transfer<'a, 'shared> {
    pub context: &'a TransferContext<'shared>,
    pub request: &'a TransferRequest<'a>,
    pub server: &'a ServerInfo,
    pub size_on_disk: u64
}
