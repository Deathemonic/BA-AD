#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::{env, fs, process};

    use baad_dm::{Download, DownloadStatus, Downloader, DownloaderConfig, Summary};
    use bacy::crypto::md5;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::task::JoinHandle;

    static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

    struct Directory(PathBuf);

    impl Directory {
        fn new() -> Self {
            let id = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = env::temp_dir().join(format!("baad-dm-integrity-{}-{id}", process::id()));
            fs::create_dir_all(&path).expect("create test directory");
            Self(path)
        }
    }

    impl Drop for Directory {
        fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
    }

    struct Server {
        url: String,
        requests: Arc<Mutex<Vec<String>>>,
        task: JoinHandle<()>
    }

    impl Server {
        async fn new(handler: impl Fn(&str) -> Vec<u8> + Send + Sync + 'static) -> Self {
            let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind server");
            let url =
                format!("http://{}/asset.bin", listener.local_addr().expect("server address"));
            let requests = Arc::new(Mutex::new(Vec::new()));
            let captured = Arc::clone(&requests);
            let handler = Arc::new(handler);
            let task = tokio::spawn(async move {
                while let Ok((mut socket, _)) = listener.accept().await {
                    let handler = Arc::clone(&handler);
                    let captured = Arc::clone(&captured);
                    tokio::spawn(async move {
                        let mut request = Vec::new();
                        let mut buffer = [0; 4096];
                        while !request.windows(4).any(|window| window == b"\r\n\r\n") {
                            let count = socket.read(&mut buffer).await.expect("read request");
                            if count == 0 {
                                return;
                            }
                            request.extend_from_slice(&buffer[..count]);
                        }
                        let request = String::from_utf8(request).expect("HTTP request");
                        captured.lock().expect("request log").push(request.clone());
                        let _ = socket.write_all(&handler(&request)).await;
                    });
                }
            });
            Self { url, requests, task }
        }

        async fn bytes(body: &'static [u8]) -> Self {
            Self::new(move |request| {
                response(
                    "200 OK",
                    body.len(),
                    "",
                    if request.starts_with("HEAD ") { &[] } else { body }
                )
            })
            .await
        }
    }

    impl Drop for Server {
        fn drop(&mut self) { self.task.abort(); }
    }

    fn response(status: &str, size: usize, headers: &str, body: &[u8]) -> Vec<u8> {
        let mut response = format!(
            "HTTP/1.1 {status}\r\nContent-Length: {size}\r\nConnection: close\r\n{headers}\r\n"
        )
        .into_bytes();
        response.extend_from_slice(body);
        response
    }

    fn hash(bytes: &[u8]) -> String { md5::to_hex_string(&md5::compute_hash(bytes)) }

    fn download(server: &Server, size: Option<u64>, expected_hash: Option<String>) -> Download {
        Download::builder()
            .url(server.url.parse().expect("download URL"))
            .filename("asset.bin".to_string())
            .maybe_size(size)
            .maybe_hash(expected_hash)
            .build()
    }

    async fn fetch(directory: &Path, download: Download, overwrite: bool) -> Summary {
        let config = DownloaderConfig::builder()
            .directory(directory)
            .overwrite(overwrite)
            .retries(0)
            .build();
        Downloader::new(config).download(&[download]).await.remove(0)
    }

    #[tokio::test]
    async fn verified_download_is_published_and_skipped_next_time() {
        let server = Server::bytes(b"valid asset").await;
        let directory = Directory::new();
        let item = download(&server, Some(11), Some(hash(b"valid asset")));
        let summary = fetch(&directory.0, item.clone(), false).await;
        assert!(summary.is_success(), "{summary:?}");
        assert_eq!(fs::read(directory.0.join("asset.bin")).expect("asset"), b"valid asset");
        let summary = fetch(&directory.0, item, false).await;
        assert!(matches!(summary.status, DownloadStatus::Skipped(_)));
        assert_eq!(server.requests.lock().expect("requests").len(), 2);
        assert_eq!(fs::read_dir(&directory.0).expect("directory").count(), 1);
    }

    #[tokio::test]
    async fn hash_mismatch_preserves_destination_when_overwriting() {
        let server = Server::bytes(b"wrong asset").await;
        let directory = Directory::new();
        fs::write(directory.0.join("asset.bin"), b"valid asset").expect("existing asset");
        let summary =
            fetch(&directory.0, download(&server, Some(11), Some(hash(b"valid asset"))), true)
                .await;
        assert!(
            matches!(&summary.status, DownloadStatus::Failed(reason) if reason.contains("hash"))
        );
        assert_eq!(fs::read(directory.0.join("asset.bin")).expect("asset"), b"valid asset");
        assert_eq!(fs::read_dir(&directory.0).expect("directory").count(), 1);
    }

    #[tokio::test]
    async fn manifest_size_mismatch_never_publishes_destination() {
        let server = Server::bytes(b"short").await;
        let directory = Directory::new();
        let summary = fetch(&directory.0, download(&server, Some(10), None), false).await;
        assert!(
            matches!(&summary.status, DownloadStatus::Failed(reason) if reason.contains("Size mismatch"))
        );
        assert!(!directory.0.join("asset.bin").exists());
        assert_eq!(fs::read_dir(&directory.0).expect("directory").count(), 0);
    }

    #[tokio::test]
    async fn existing_wrong_size_is_downloaded_again_without_a_hash() {
        let server = Server::bytes(b"valid asset").await;
        let directory = Directory::new();
        fs::write(directory.0.join("asset.bin"), b"short").expect("existing asset");
        let summary = fetch(&directory.0, download(&server, Some(11), None), false).await;
        assert!(summary.is_success(), "{summary:?}");
        assert_eq!(fs::read(directory.0.join("asset.bin")).expect("asset"), b"valid asset");
    }

    #[tokio::test]
    async fn failed_replacement_keeps_existing_file() {
        let server = Server::new(|_| response("404 Not Found", 0, "", &[])).await;
        let directory = Directory::new();
        fs::write(directory.0.join("asset.bin"), b"old").expect("existing asset");
        let summary =
            fetch(&directory.0, download(&server, Some(11), Some(hash(b"valid asset"))), false)
                .await;
        assert!(matches!(summary.status, DownloadStatus::Failed(_)));
        assert_eq!(fs::read(directory.0.join("asset.bin")).expect("asset"), b"old");
    }

    #[tokio::test]
    async fn unsupported_head_does_not_validate_against_error_page_size() {
        let server = Server::new(|request| {
            if request.starts_with("HEAD ") {
                response("405 Method Not Allowed", 0, "", &[])
            } else {
                response("200 OK", 11, "", b"valid asset")
            }
        })
        .await;
        let directory = Directory::new();
        let summary = fetch(&directory.0, download(&server, None, None), false).await;
        assert!(summary.is_success(), "{summary:?}");
        assert_eq!(fs::read(directory.0.join("asset.bin")).expect("asset"), b"valid asset");
    }

    async fn resumed_download(ignore_range: bool) {
        let body = Arc::new(vec![42; 1024 * 1024]);
        let expected_hash = hash(&body);
        let get_count = AtomicUsize::new(0);
        let server_body = Arc::clone(&body);
        let server = Server::new(move |request| {
            let body = &server_body;
            if request.starts_with("HEAD ") {
                return response("200 OK", body.len(), "Accept-Ranges: bytes\r\n", &[]);
            }
            if get_count.fetch_add(1, Ordering::Relaxed) == 0 {
                return response("200 OK", body.len(), "", &body[..600_000]);
            }
            let range = request
                .lines()
                .find_map(|line| line.strip_prefix("range: bytes="))
                .expect("resume range");
            let start = range.trim_end_matches('-').parse::<usize>().expect("range start");
            assert!(start > 0 && start < body.len());
            if ignore_range {
                response("200 OK", body.len(), "", body)
            } else {
                response(
                    "206 Partial Content",
                    body.len() - start,
                    &format!("Content-Range: bytes {start}-{}/{}\r\n", body.len() - 1, body.len()),
                    &body[start..]
                )
            }
        })
        .await;
        let directory = Directory::new();
        let item = download(&server, Some(body.len() as u64), Some(expected_hash));
        let summary = fetch(&directory.0, item.clone(), false).await;
        assert!(matches!(summary.status, DownloadStatus::Failed(_)));
        assert!(!directory.0.join("asset.bin").exists());
        let summary = fetch(&directory.0, item, false).await;
        assert!(
            server
                .requests
                .lock()
                .expect("requests")
                .iter()
                .filter(|request| request.starts_with("GET "))
                .nth(1)
                .expect("second GET")
                .contains("range: bytes=")
        );
        if ignore_range {
            assert!(matches!(summary.status, DownloadStatus::Failed(_)));
            assert!(!directory.0.join("asset.bin").exists());
        } else {
            assert!(summary.is_success(), "{summary:?}");
            assert_eq!(fs::read(directory.0.join("asset.bin")).expect("asset"), *body);
        }
    }

    #[tokio::test]
    async fn interrupted_stream_resumes_and_validates_before_publishing() {
        resumed_download(false).await;
    }

    #[tokio::test]
    async fn ignored_resume_range_is_rejected_without_publishing() { resumed_download(true).await; }

    #[tokio::test]
    async fn interrupted_unverifiable_or_nonresumable_stream_discards_staging_file() {
        for (with_hash, with_ranges) in [(false, true), (true, false)] {
            let body = vec![42; 1024 * 1024];
            let expected_hash = with_hash.then(|| hash(&body));
            let server = Server::new(move |request| {
                if request.starts_with("HEAD ") {
                    let headers = if with_ranges { "Accept-Ranges: bytes\r\n" } else { "" };
                    response("200 OK", body.len(), headers, &[])
                } else {
                    response("200 OK", body.len(), "", &body[..600_000])
                }
            })
            .await;
            let directory = Directory::new();
            let summary =
                fetch(&directory.0, download(&server, Some(1024 * 1024), expected_hash), false)
                    .await;
            assert!(matches!(summary.status, DownloadStatus::Failed(_)));
            assert_eq!(fs::read_dir(&directory.0).expect("directory").count(), 0);
        }
    }

    async fn chunked_download(mode: &str) {
        // The production chunk size is 8 MiB, so 16 MiB exercises two ranges.
        let body = Arc::new(vec![42; 16 * 1024 * 1024]);
        let server_body = Arc::clone(&body);
        let mode = mode.to_string();
        let should_succeed = mode == "valid";
        let server = Server::new(move |request| {
            let body = &server_body;
            if request.starts_with("HEAD ") {
                return response("200 OK", body.len(), "Accept-Ranges: bytes\r\n", &[]);
            }
            if mode == "ignored" {
                return response("200 OK", body.len(), "", body);
            }
            let range = request
                .lines()
                .find_map(|line| line.strip_prefix("range: bytes="))
                .expect("chunk range");
            let (start, end) = range.split_once('-').expect("range bounds");
            let start = start.parse::<usize>().expect("range start");
            let end = end.parse::<usize>().expect("range end");
            let reported_start = if mode == "wrong-range" { start + 1 } else { start };
            let headers = format!("Content-Range: bytes {reported_start}-{end}/{}\r\n", body.len());
            let actual_end = match mode.as_str() {
                "short" => start,
                "long" if end < body.len() - 1 => end + 1,
                _ => end
            };
            response(
                "206 Partial Content",
                actual_end - start + 1,
                &headers,
                &body[start..=actual_end]
            )
        })
        .await;
        let directory = Directory::new();
        let summary =
            fetch(&directory.0, download(&server, Some(body.len() as u64), None), false).await;
        if should_succeed {
            assert!(summary.is_success(), "{summary:?}");
            assert_eq!(fs::read(directory.0.join("asset.bin")).expect("asset"), *body);
        } else {
            assert!(matches!(summary.status, DownloadStatus::Failed(_)));
            assert!(!directory.0.join("asset.bin").exists());
            assert_eq!(fs::read_dir(&directory.0).expect("directory").count(), 0);
        }
    }

    #[tokio::test]
    async fn valid_chunk_ranges_publish_complete_file() { chunked_download("valid").await; }

    #[tokio::test]
    async fn ignored_chunk_ranges_cannot_be_reported_as_success() {
        chunked_download("ignored").await;
    }

    #[tokio::test]
    async fn short_chunks_cannot_hide_in_preallocated_file() { chunked_download("short").await; }

    #[tokio::test]
    async fn oversized_chunks_cannot_overwrite_neighboring_ranges() {
        chunked_download("long").await;
    }

    #[tokio::test]
    async fn incorrect_content_range_is_rejected() { chunked_download("wrong-range").await; }

    #[tokio::test]
    async fn zip_member_is_not_checked_against_archive_size_or_hash() {
        // A stored ZIP containing member.txt with the text "extracted asset".
        const ARCHIVE: &[u8] = b"\x50\x4b\x03\x04\x14\x00\x00\x00\x00\x00\x00\x00\x21\x5c\x89\x56\x76\xef\x0f\x00\x00\x00\x0f\x00\x00\x00\x0a\x00\x00\x00\x6d\x65\x6d\x62\x65\x72\x2e\x74\x78\x74\x65\x78\x74\x72\x61\x63\x74\x65\x64\x20\x61\x73\x73\x65\x74\x50\x4b\x01\x02\x14\x03\x14\x00\x00\x00\x00\x00\x00\x00\x21\x5c\x89\x56\x76\xef\x0f\x00\x00\x00\x0f\x00\x00\x00\x0a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x80\x01\x00\x00\x00\x00\x6d\x65\x6d\x62\x65\x72\x2e\x74\x78\x74\x50\x4b\x05\x06\x00\x00\x00\x00\x01\x00\x01\x00\x38\x00\x00\x00\x37\x00\x00\x00\x00\x00";
        let server = Server::new(|request| {
            if request.starts_with("HEAD ") {
                return response("200 OK", ARCHIVE.len(), "Accept-Ranges: bytes\r\n", &[]);
            }
            let range = request
                .lines()
                .find_map(|line| line.strip_prefix("range: bytes="))
                .expect("ZIP range");
            let (start, end) = range.split_once('-').expect("range bounds");
            let start = start.parse::<usize>().expect("range start");
            let end = end.parse::<usize>().expect("range end");
            response(
                "206 Partial Content",
                end - start + 1,
                &format!("Content-Range: bytes {start}-{end}/{}\r\n", ARCHIVE.len()),
                &ARCHIVE[start..=end]
            )
        })
        .await;
        let directory = Directory::new();
        let mut item = download(&server, Some(ARCHIVE.len() as u64), Some(hash(ARCHIVE)));
        item.target_file = Some("member.txt".to_string());
        let summary = fetch(&directory.0, item, false).await;
        assert!(summary.is_success(), "{summary:?}");
        assert_eq!(
            fs::read(directory.0.join("asset.bin")).expect("extracted member"),
            b"extracted asset"
        );
    }
}
