use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};

use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::reqwest::Url;
use tokio::sync::OnceCell;

use crate::error::Error;
use crate::zip::{ZipExtractor, ZipIndex};

type IndexCell = Arc<OnceCell<Arc<ZipIndex>>>;

#[derive(Clone, Default, Debug)]
pub struct ZipCache {
    inner: Arc<Mutex<HashMap<String, IndexCell>>>
}

impl ZipCache {
    pub fn new() -> Self { Self::default() }

    pub async fn get_index(
        &self,
        client: &ClientWithMiddleware,
        url: &Url
    ) -> Result<Arc<ZipIndex>, Error> {
        let cell = {
            let mut map = self.inner.lock().unwrap_or_else(PoisonError::into_inner);

            Arc::clone(map.entry(url.as_str().into()).or_default())
        };

        cell.get_or_try_init(|| async {
            let extractor = ZipExtractor::new(client, url).await?;
            Ok::<_, Error>(Arc::new(extractor.build_index().await?))
        })
        .await
        .cloned()
    }
}
