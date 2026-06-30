use std::path::{Path, PathBuf};

use baad_shared::{CatalogError, client};
use baad_utils::debug;
use memorypack::{MemoryPackDeserialize, MemoryPackSerialize, MemoryPackSerializer};
use tokio::fs;

use crate::download::download_file;

pub struct CatalogFile {
    pub url: String,
    pub hash_url: String,
    pub path: PathBuf
}

impl CatalogFile {
    pub fn pack_path(&self) -> PathBuf { self.path.with_extension("bytes") }

    fn name(&self) -> &str {
        self.path.file_name().and_then(|name| name.to_str()).unwrap_or_default()
    }
}

pub async fn ensure_cached(file: &CatalogFile) -> Result<bool, CatalogError> {
    let filename = file.name();
    let hash_path = file.path.with_extension("hash");

    let remote = remote_hash(&file.hash_url).await;
    let local = local_hash(&hash_path).await;

    let outdated = !file.path.exists()
        || match (&remote, &local) {
            (Some(remote), Some(local)) => remote != local,
            (Some(_), None) => true,
            (None, _) => false
        };

    if outdated {
        debug!(filename, "Catalog outdated, fetching...");
        download_file(&file.url, &file.path, None, 3).await?;
        if let Some(remote) = &remote {
            fs::write(&hash_path, remote).await?;
        }
    } else {
        debug!(filename, "Catalog up to date, using cache");
    }

    Ok(outdated)
}

pub async fn remote_hash(url: &str) -> Option<String> {
    let response = client().get(url).send().await.ok()?.error_for_status().ok()?;
    let text = response.text().await.ok()?;
    Some(text.trim().to_string())
}

pub async fn local_hash(path: &Path) -> Option<String> {
    fs::read_to_string(path).await.ok().map(|hash| hash.trim().to_string())
}

pub async fn read_pack<T: MemoryPackDeserialize>(path: &Path) -> Option<T> {
    let bytes = fs::read(path).await.ok()?;
    MemoryPackSerializer::deserialize::<T>(&bytes).ok()
}

pub async fn write_pack<T: MemoryPackSerialize>(
    path: &Path,
    value: &T
) -> Result<(), CatalogError> {
    let bytes = MemoryPackSerializer::serialize(value)?;
    fs::write(path, bytes).await?;
    Ok(())
}
