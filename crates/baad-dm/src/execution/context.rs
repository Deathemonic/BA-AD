use std::sync::Arc;

use reqwest_middleware::ClientWithMiddleware;
use tokio::sync::Semaphore;

use crate::zip::ZipCache;

#[derive(Debug)]
pub struct SharedExecution {
    pub client: ClientWithMiddleware,
    pub requests: Arc<Semaphore>,
    pub transforms: Arc<Semaphore>,
    pub blocking: Arc<Semaphore>,
    pub zip_cache: ZipCache
}

impl SharedExecution {
    pub fn new(client: ClientWithMiddleware, max_requests: usize, max_transforms: usize) -> Self {
        Self {
            client,
            requests: Arc::new(Semaphore::new(max_requests)),
            transforms: Arc::new(Semaphore::new(max_transforms)),
            blocking: Arc::new(Semaphore::new(max_requests)),
            zip_cache: ZipCache::new()
        }
    }
}
