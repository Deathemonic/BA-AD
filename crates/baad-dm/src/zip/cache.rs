use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
            let mut map = self.inner.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            match map.get(url.as_str()) {
                Some(cell) => cell.clone(),
                None => {
                    let cell = Arc::new(OnceCell::new());
                    map.insert(url.as_str().into(), cell.clone());
                    cell
                }
            }
        };

        cell.get_or_try_init(|| async {
            let extractor = ZipExtractor::new(client, url).await?;
            Ok::<_, Error>(Arc::new(extractor.build_index().await?))
        })
        .await
        .cloned()
    }
}
