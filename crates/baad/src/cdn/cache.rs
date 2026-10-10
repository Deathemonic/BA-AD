use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::{fs as std_fs, io, process};

use baad_shared::client;
use bacy::crypto::md5;
use memorypack::{MemoryPackDeserialize, MemoryPackSerialize, MemoryPackSerializer};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tracing::{debug, warn};

use crate::download::download_file;
use crate::error::CatalogError;

const PACK_MAGIC: &[u8] = b"BAADPACK\x01";
static TEMP_ID: AtomicU64 = AtomicU64::new(0);

pub struct CatalogFile {
    pub url: String,
    pub hash_url: Option<String>,
    pub path: PathBuf
}

impl CatalogFile {
    pub fn pack_path(&self) -> PathBuf { self.path.with_extension("bytes") }

    fn metadata_path(&self) -> PathBuf { self.path.with_extension("cache.json") }

    async fn invalidate(&self) -> Result<(), CatalogError> {
        match fs::remove_file(self.metadata_path()).await {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into())
        }
    }

    fn name(&self) -> &str {
        self.path.file_name().and_then(|name| name.to_str()).unwrap_or_default()
    }
}

#[derive(Deserialize, Serialize)]
struct Metadata {
    url: String,
    remote_hash: Option<String>,
    source_hash: String
}

/// A cached catalog source together with its MD5 digest, so callers can bind
/// packs to it without reading or hashing the file again.
pub(crate) struct Source {
    pub bytes: Vec<u8>,
    pub digest: [u8; 16]
}

impl Source {
    fn new(bytes: Vec<u8>) -> Self {
        let digest = md5::compute_hash(&bytes);
        Self { bytes, digest }
    }

    fn hex(&self) -> String { md5::to_hex_string(&self.digest) }
}

pub async fn ensure_cached(file: &CatalogFile) -> Result<bool, CatalogError> {
    refresh(file).await.map(|(downloaded, _)| downloaded)
}

pub(crate) async fn load(file: &CatalogFile) -> Result<Source, CatalogError> {
    refresh(file).await.map(|(_, source)| source)
}

async fn refresh(file: &CatalogFile) -> Result<(bool, Source), CatalogError> {
    let filename = file.name();
    let remote = match file.hash_url.as_deref() {
        Some(url) => remote_hash(url).await,
        None => None
    };
    let metadata = fs::read(file.metadata_path())
        .await
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Metadata>(&bytes).ok());
    let source = fs::read(&file.path).await.ok().map(Source::new);

    // The remote hash alone cannot identify the source after a version/URL
    // change, nor prove that a previous download and metadata update both
    // completed.
    if let Some(source) = source
        && metadata.as_ref().is_some_and(|metadata| {
            metadata.url == file.url
                && metadata.source_hash == source.hex()
                && remote.as_ref().is_none_or(|hash| metadata.remote_hash.as_ref() == Some(hash))
        })
    {
        if file.hash_url.is_some() && remote.is_none() {
            warn!(filename, "Catalog hash unavailable, using verified cache for the same URL");
        }
        debug!(filename, "Catalog up to date, using cache");
        return Ok((false, source));
    }

    debug!(filename, "Catalog outdated, fetching...");
    let temporary = TemporaryFile::new(&file.path).await?;
    download_file(&file.url, &temporary.path, None, 5).await?;
    let source = Source::new(fs::read(&temporary.path).await?);
    let metadata = Metadata {
        url: file.url.clone(),
        remote_hash: remote,
        source_hash: source.hex()
    };
    fs::rename(&temporary.path, &file.path).await?;
    // If interrupted between these writes, the source digest will not match and
    // the next run will fetch again instead of trusting an inconsistent cache.
    write_atomic(&file.metadata_path(), &serde_json::to_vec(&metadata)?).await?;
    // Markers written by earlier versions of the cache are no longer read.
    for legacy in ["hash", "url"] {
        let _ = fs::remove_file(file.path.with_extension(legacy)).await;
    }
    Ok((true, source))
}

pub async fn remote_hash(url: &str) -> Option<String> {
    let response = client().get(url).send().await.ok()?.error_for_status().ok()?;
    let text = response.text().await.ok()?;
    let hash = text.trim();
    (!hash.is_empty()).then(|| hash.into())
}

pub(crate) async fn fetch_json<T>(file: &CatalogFile) -> Result<T, CatalogError>
where
    T: DeserializeOwned + MemoryPackSerialize + MemoryPackDeserialize
{
    let source = load(file).await?;
    let pack = file.pack_path();
    if let Some(value) = read_bound_pack::<T>(&pack, &source.digest).await {
        return Ok(value);
    }

    let value = match serde_json::from_slice::<T>(&source.bytes) {
        Ok(value) => value,
        Err(error) => {
            // An HTTP 200 can still contain an invalid catalog. Make the next
            // attempt refetch it, including URL-only catalogs without a hash.
            file.invalidate().await?;
            return Err(error.into());
        }
    };
    write_bound_pack(&pack, &source.digest, &value).await?;
    Ok(value)
}

pub(crate) async fn fetch_memorypack<T: MemoryPackDeserialize>(
    file: &CatalogFile
) -> Result<T, CatalogError> {
    let source = load(file).await?;
    match MemoryPackSerializer::deserialize::<T>(&source.bytes) {
        Ok(value) => Ok(value),
        Err(error) => {
            file.invalidate().await?;
            Err(error.into())
        }
    }
}

pub async fn read_pack<T: MemoryPackDeserialize>(path: &Path, source: &[u8]) -> Option<T> {
    read_bound_pack(path, &md5::compute_hash(source)).await
}

pub async fn write_pack<T: MemoryPackSerialize>(
    path: &Path,
    source: &[u8],
    value: &T
) -> Result<(), CatalogError> {
    write_bound_pack(path, &md5::compute_hash(source), value).await
}

pub(crate) async fn read_bound_pack<T: MemoryPackDeserialize>(
    path: &Path,
    digest: &[u8; 16]
) -> Option<T> {
    let bytes = fs::read(path).await.ok()?;
    let payload = bytes.strip_prefix(PACK_MAGIC)?;
    let payload = payload.strip_prefix(digest.as_slice())?;
    MemoryPackSerializer::deserialize::<T>(payload).ok()
}

pub(crate) async fn write_bound_pack<T: MemoryPackSerialize>(
    path: &Path,
    digest: &[u8; 16],
    value: &T
) -> Result<(), CatalogError> {
    // Keep the binding and payload in a single atomic file: updating a separate
    // marker first could incorrectly bless the old pack after an interruption.
    let mut bytes = PACK_MAGIC.to_vec();
    bytes.extend_from_slice(digest);
    bytes.extend_from_slice(&MemoryPackSerializer::serialize(value)?);
    write_atomic(path, &bytes).await
}

async fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), CatalogError> {
    let temporary = TemporaryFile::new(path).await?;
    let mut output = fs::File::create(&temporary.path).await?;
    output.write_all(bytes).await?;
    output.flush().await?;
    drop(output);
    fs::rename(&temporary.path, path).await?;
    Ok(())
}

struct TemporaryFile {
    path: PathBuf
}

impl TemporaryFile {
    async fn new(destination: &Path) -> io::Result<Self> {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).await?;
        }
        loop {
            let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
            let mut name = destination.as_os_str().to_os_string();
            name.push(format!(".tmp-{}-{id}", process::id()));
            let path = PathBuf::from(name);
            match fs::OpenOptions::new().write(true).create_new(true).open(&path).await {
                Ok(_) => return Ok(Self { path }),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error)
            }
        }
    }
}

impl Drop for TemporaryFile {
    fn drop(&mut self) { let _ = std_fs::remove_file(&self.path); }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;
    use std::sync::{Arc, Mutex};

    use tokio::io::AsyncReadExt;
    use tokio::net::TcpListener;
    use tokio::task::JoinHandle;

    use super::*;

    type Responses = Arc<Mutex<HashMap<String, (&'static str, Vec<u8>)>>>;

    struct Server {
        base_url: String,
        responses: Responses,
        task: JoinHandle<()>
    }

    impl Server {
        async fn new() -> Self {
            let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind local test server");
            let base_url =
                format!("http://{}", listener.local_addr().expect("read server address"));
            let responses = Responses::default();
            let routes = Arc::clone(&responses);
            let task = tokio::spawn(async move {
                loop {
                    let (mut stream, _) = listener.accept().await.expect("accept catalog request");
                    let mut request = Vec::new();
                    let mut buffer = [0; 1024];
                    while !request.windows(4).any(|bytes| bytes == b"\r\n\r\n") {
                        let length = stream.read(&mut buffer).await.expect("read catalog request");
                        if length == 0 {
                            break;
                        }
                        request.extend_from_slice(&buffer[..length]);
                    }
                    let request = String::from_utf8_lossy(&request);
                    let mut fields = request.split_whitespace();
                    let method = fields.next().unwrap_or_default();
                    let path = fields.next().unwrap_or_default();
                    let (status, body) = routes
                        .lock()
                        .expect("catalog response lock is not poisoned")
                        .get(path)
                        .cloned()
                        .unwrap_or(("404 Not Found", Vec::new()));
                    let response = format!(
                        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    );
                    stream.write_all(response.as_bytes()).await.expect("write response headers");
                    if method != "HEAD" {
                        stream.write_all(&body).await.expect("write catalog response");
                    }
                }
            });
            Self { base_url, responses, task }
        }

        fn serve(&self, path: &str, body: impl AsRef<[u8]>) {
            self.responses
                .lock()
                .expect("catalog response lock is not poisoned")
                .insert(path.into(), ("200 OK", body.as_ref().to_vec()));
        }

        fn catalog(&self, directory: &Directory, route: &str) -> CatalogFile {
            CatalogFile {
                url: format!("{}{route}", self.base_url),
                hash_url: Some(format!("{}{route}.hash", self.base_url)),
                path: directory.0.join("catalog.json")
            }
        }
    }

    impl Drop for Server {
        fn drop(&mut self) { self.task.abort(); }
    }

    struct Directory(PathBuf);

    impl Directory {
        fn new() -> Self {
            let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
            let path = env::temp_dir().join(format!("baad-catalog-{}-{id}", process::id()));
            std_fs::create_dir_all(&path).expect("create test cache directory");
            Self(path)
        }
    }

    impl Drop for Directory {
        fn drop(&mut self) { let _ = std_fs::remove_dir_all(&self.0); }
    }

    #[tokio::test]
    async fn interrupted_refresh_rejects_old_pack_after_metadata_update() -> Result<(), CatalogError>
    {
        let server = Server::new().await;
        let directory = Directory::new();
        let file = server.catalog(&directory, "/catalog");
        server.serve("/catalog", "old source");
        server.serve("/catalog.hash", "version-1");
        assert!(ensure_cached(&file).await?);
        write_pack(&file.pack_path(), b"old source", &vec![1_u32]).await?;
        assert_eq!(read_pack::<Vec<u32>>(&file.pack_path(), b"old source").await, Some(vec![1]));

        server.serve("/catalog", "new source");
        server.serve("/catalog.hash", "version-2");
        assert!(ensure_cached(&file).await?);
        // Simulate exit after source/metadata publication, before writing a
        // pack.
        assert!(!ensure_cached(&file).await?);
        let source = fs::read(&file.path).await?;
        assert_eq!(source, b"new source");
        assert!(read_pack::<Vec<u32>>(&file.pack_path(), &source).await.is_none());
        write_pack(&file.pack_path(), &source, &vec![2_u32]).await?;
        assert_eq!(read_pack::<Vec<u32>>(&file.pack_path(), &source).await, Some(vec![2]));
        Ok(())
    }

    #[tokio::test]
    async fn interrupted_source_update_is_detected_without_remote_hash() -> Result<(), CatalogError>
    {
        let server = Server::new().await;
        let directory = Directory::new();
        let file = server.catalog(&directory, "/catalog");
        server.serve("/catalog", "old source");
        assert!(ensure_cached(&file).await?);
        // Source was published, but its metadata was never updated.
        fs::write(&file.path, b"interrupted source").await?;
        server.serve("/catalog", "new source");
        assert!(ensure_cached(&file).await?);
        assert_eq!(fs::read(&file.path).await?, b"new source");
        assert!(!ensure_cached(&file).await?);
        Ok(())
    }

    #[tokio::test]
    async fn changed_url_fetches_new_source_when_hash_endpoint_fails() -> Result<(), CatalogError> {
        let server = Server::new().await;
        let directory = Directory::new();
        server.serve("/version-1/catalog", "old source");
        server.serve("/version-2/catalog", "new source");
        let old_file = server.catalog(&directory, "/version-1/catalog");
        assert!(ensure_cached(&old_file).await?);
        let new_file = server.catalog(&directory, "/version-2/catalog");
        assert!(ensure_cached(&new_file).await?);
        assert_eq!(fs::read(&new_file.path).await?, b"new source");
        assert!(!ensure_cached(&new_file).await?);
        Ok(())
    }

    #[tokio::test]
    async fn failed_refresh_preserves_existing_source_and_metadata() -> Result<(), CatalogError> {
        let server = Server::new().await;
        let directory = Directory::new();
        server.serve("/version-1/catalog", "old source");
        let old_file = server.catalog(&directory, "/version-1/catalog");
        assert!(ensure_cached(&old_file).await?);
        let old_metadata = fs::read(old_file.metadata_path()).await?;
        let new_file = server.catalog(&directory, "/missing/catalog");
        assert!(ensure_cached(&new_file).await.is_err());
        assert_eq!(fs::read(&old_file.path).await?, b"old source");
        assert_eq!(fs::read(old_file.metadata_path()).await?, old_metadata);
        Ok(())
    }

    #[tokio::test]
    async fn global_url_only_cache_binds_pack_to_current_source() -> Result<(), CatalogError> {
        let server = Server::new().await;
        let directory = Directory::new();
        server.serve("/version-1/catalog", "old source");
        server.serve("/version-2/catalog", "new source");
        let mut file = server.catalog(&directory, "/version-1/catalog");
        file.hash_url = None;
        assert!(ensure_cached(&file).await?);
        write_pack(&file.pack_path(), b"old source", &vec![1_u32]).await?;
        file.url = format!("{}/version-2/catalog", server.base_url);
        assert!(ensure_cached(&file).await?);
        assert!(!ensure_cached(&file).await?);
        let source = fs::read(&file.path).await?;
        assert!(read_pack::<Vec<u32>>(&file.pack_path(), &source).await.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn unbound_legacy_pack_is_rebuilt() -> Result<(), CatalogError> {
        let directory = Directory::new();
        let path = directory.0.join("catalog.bytes");
        fs::write(&path, MemoryPackSerializer::serialize(&vec![1_u32])?).await?;
        assert!(read_pack::<Vec<u32>>(&path, b"source").await.is_none());
        write_pack(&path, b"source", &vec![2_u32]).await?;
        assert_eq!(read_pack::<Vec<u32>>(&path, b"source").await, Some(vec![2]));
        Ok(())
    }

    async fn json_error_refetches_same_url(has_hash_endpoint: bool) -> Result<(), CatalogError> {
        let server = Server::new().await;
        let directory = Directory::new();
        let mut file = server.catalog(&directory, "/catalog");
        if has_hash_endpoint {
            server.serve("/catalog.hash", "version-1");
        } else {
            file.hash_url = None;
        }
        server.serve("/catalog", "<html>temporarily unavailable</html>");
        assert!(matches!(fetch_json::<Vec<u32>>(&file).await, Err(CatalogError::SerdeJson(_))));
        assert!(!file.metadata_path().exists());

        // The URL and hash have not changed; the invalid response must not pin
        // this source in the cache after the server starts returning valid
        // JSON.
        server.serve("/catalog", "[2]");
        assert_eq!(fetch_json::<Vec<u32>>(&file).await?, vec![2]);
        assert!(!ensure_cached(&file).await?);
        Ok(())
    }

    #[tokio::test]
    async fn global_json_parse_error_refetches_same_url() -> Result<(), CatalogError> {
        json_error_refetches_same_url(false).await?;
        Ok(())
    }

    #[tokio::test]
    async fn china_json_parse_error_refetches_same_url_and_hash() -> Result<(), CatalogError> {
        json_error_refetches_same_url(true).await?;
        Ok(())
    }

    #[tokio::test]
    async fn japan_memorypack_parse_error_refetches_same_url_and_hash() -> Result<(), CatalogError>
    {
        let server = Server::new().await;
        let directory = Directory::new();
        let file = server.catalog(&directory, "/catalog");
        server.serve("/catalog.hash", "version-1");
        // A vector length followed by a truncated element.
        server.serve("/catalog", [1, 0, 0, 0]);
        assert!(matches!(
            fetch_memorypack::<Vec<u32>>(&file).await,
            Err(CatalogError::MemoryPack(_))
        ));
        assert!(!file.metadata_path().exists());

        server.serve("/catalog", MemoryPackSerializer::serialize(&vec![2_u32])?);
        assert_eq!(fetch_memorypack::<Vec<u32>>(&file).await?, vec![2]);
        assert!(!ensure_cached(&file).await?);
        Ok(())
    }
}
