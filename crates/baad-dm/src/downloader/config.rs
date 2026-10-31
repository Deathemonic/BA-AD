use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::Arc;

use baad_shared::ProgressObserver;
use bon::Builder;
use derive_more::Debug;
use reqwest_middleware::reqwest::Proxy;
use reqwest_middleware::reqwest::header::HeaderMap;

use crate::error::Error;

#[derive(Builder, Clone, Debug)]
pub struct DownloaderConfig<'a> {
    pub directory: &'a Path,

    #[builder(default = NonZeroUsize::new(32).expect("32 is nonzero"))]
    #[builder(with = |value: usize| -> Result<_, Error> { validate_concurrency(value) })]
    pub concurrent_downloads: NonZeroUsize,

    #[builder(default = NonZeroUsize::new(16).expect("16 is nonzero"))]
    #[builder(with = |value: usize| -> Result<_, Error> { validate_concurrency(value) })]
    pub max_chunks_per_file: NonZeroUsize,

    #[builder(default = NonZeroUsize::new(8).expect("8 is nonzero"))]
    #[builder(with = |value: usize| -> Result<_, Error> { validate_concurrency(value) })]
    pub max_concurrent_chunks: NonZeroUsize,

    #[builder(default = NonZeroUsize::new(4).expect("4 is nonzero"))]
    #[builder(with = |value: usize| -> Result<_, Error> { validate_concurrency(value) })]
    pub max_concurrent_segments: NonZeroUsize,

    #[builder(default = NonZeroUsize::new(32).expect("32 is nonzero"))]
    #[builder(with = |value: usize| -> Result<_, Error> { validate_concurrency(value) })]
    pub max_concurrent_requests: NonZeroUsize,

    #[builder(default = NonZeroUsize::new(2).expect("2 is nonzero"))]
    #[builder(with = |value: usize| -> Result<_, Error> { validate_concurrency(value) })]
    pub max_concurrent_transforms: NonZeroUsize,

    #[builder(default = 10 * 1024 * 1024)]
    pub chunk_threshold: u64,

    #[builder(default = 3)]
    pub retries: u32,

    #[builder(default = true)]
    pub resumable: bool,

    #[builder(default = false)]
    pub overwrite: bool,

    #[builder(default = false)]
    pub http1_only: bool,

    #[builder(default = true)]
    pub log_results: bool,

    #[debug(skip)]
    pub headers: Option<HeaderMap>,

    #[debug(skip)]
    pub proxy: Option<Proxy>,

    #[debug(skip)]
    #[builder(default = baad_shared::observer())]
    pub observer: Arc<dyn ProgressObserver>
}

fn validate_concurrency(value: usize) -> Result<NonZeroUsize, Error> {
    NonZeroUsize::new(value).ok_or(Error::ZeroConcurrency)
}
