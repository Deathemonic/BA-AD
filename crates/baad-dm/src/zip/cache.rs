use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};

use tokio::sync::OnceCell;

use crate::error::Error;
use crate::zip::{ZipExtractor, ZipIndex, ZipSource};

type IndexCell = Arc<OnceCell<Arc<ZipIndex>>>;

#[derive(Clone, Default, Debug)]
pub struct ZipCache {
    inner: Arc<Mutex<HashMap<String, IndexCell>>>
}

impl ZipCache {
    pub fn new() -> Self { Self::default() }

    pub async fn get_index(&self, source: ZipSource<'_>) -> Result<Arc<ZipIndex>, Error> {
        let cell = {
            let mut map = self.inner.lock().unwrap_or_else(PoisonError::into_inner);

            Arc::clone(map.entry(source.url.as_str().into()).or_default())
        };

        cell.get_or_try_init(|| async {
            let extractor = ZipExtractor::new(source).await?;
            Ok::<_, Error>(Arc::new(extractor.build_index().await?))
        })
        .await
        .cloned()
    }
}
