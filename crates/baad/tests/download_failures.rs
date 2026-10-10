#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::{env, fs, io, process};

    use baad::download::{ExpectedFile, ResourceDownloader, download_expected_file, download_file};
    use baad_dm::HashType;
    use baad_shared::{DownloadAsset, DownloadMedia, Downloads, HashValue};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::{TcpListener, TcpStream};
    use tokio::task::JoinHandle;

    const GOOD_MD5: &str = "755f85c2723bb39381c7379a604160d8";
    // CRC-64/XZ of b"good".
    const GOOD_CRC64: &str = "578325586736611102";
    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct Fixture {
        directory: PathBuf,
        base_url: String,
        server: JoinHandle<()>
    }

    impl Fixture {
        async fn new() -> io::Result<Self> {
            let directory = env::temp_dir().join(format!(
                "baad-download-failures-{}-{}",
                process::id(),
                NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&directory)?;
            let listener = TcpListener::bind("127.0.0.1:0").await?;
            let base_url = format!("http://{}", listener.local_addr()?);
            let server = tokio::spawn(async move {
                while let Ok((stream, _)) = listener.accept().await {
                    let _ = respond(stream).await;
                }
            });
            Ok(Self {
                directory,
                base_url,
                server
            })
        }

        fn media(&self, name: &str) -> DownloadMedia {
            DownloadMedia {
                url: format!("{}/{name}", self.base_url),
                path: name.into(),
                hash: HashValue::Md5(GOOD_MD5.into()),
                size: 4
            }
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            self.server.abort();
            let _ = fs::remove_dir_all(&self.directory);
        }
    }

    async fn respond(mut stream: TcpStream) -> io::Result<()> {
        let mut request = Vec::new();
        let mut buffer = [0_u8; 1024];
        while !request.windows(4).any(|bytes| bytes == b"\r\n\r\n") {
            let size = stream.read(&mut buffer).await?;
            if size == 0 || request.len() > 8192 {
                return Ok(());
            }
            request.extend_from_slice(&buffer[..size]);
        }
        let request = String::from_utf8_lossy(&request);
        let status = if request.split_whitespace().nth(1) == Some("/ok.bin") {
            "200 OK"
        } else {
            "404 Not Found"
        };
        let headers =
            format!("HTTP/1.1 {status}\r\nContent-Length: 4\r\nConnection: close\r\n\r\n");
        stream.write_all(headers.as_bytes()).await?;
        if !request.starts_with("HEAD ") {
            stream.write_all(b"good").await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn reports_failures_across_categories_and_keeps_successful_files()
    -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new().await?;
        let downloads = Downloads {
            assets: vec![DownloadAsset {
                url: format!("{}/missing.asset", fixture.base_url),
                path: "missing.asset".into(),
                hash: HashValue::Md5(GOOD_MD5.into()),
                size: 4,
                bundle_files: Vec::new()
            }],
            tables: Vec::new(),
            media: vec![fixture.media("missing.ogg"), fixture.media("ok.bin")]
        };
        let result = ResourceDownloader::builder()
            .output_dir(fixture.directory.clone())
            .retries(0)
            .build()
            .download(downloads, None)
            .await;
        let error = result.expect_err("A failed batch must return an error").to_string();
        assert!(error.contains("missing.asset"), "{error}");
        assert!(error.contains("missing.ogg"), "{error}");
        assert!(error.contains("404"), "{error}");
        assert!(!fixture.directory.join("missing.asset").exists());
        assert!(!fixture.directory.join("missing.ogg").exists());
        assert_eq!(fs::read(fixture.directory.join("ok.bin"))?, b"good");
        Ok(())
    }

    #[tokio::test]
    async fn successful_batch_returns_success() -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new().await?;
        ResourceDownloader::builder()
            .output_dir(fixture.directory.clone())
            .retries(0)
            .build()
            .download(
                Downloads {
                    assets: Vec::new(),
                    tables: Vec::new(),
                    media: vec![fixture.media("ok.bin")]
                },
                None
            )
            .await?;
        assert_eq!(fs::read(fixture.directory.join("ok.bin"))?, b"good");
        Ok(())
    }

    #[tokio::test]
    async fn single_file_failure_preserves_filename_and_http_error() -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new().await?;
        let error = download_file(
            &format!("{}/missing.json", fixture.base_url),
            &fixture.directory.join("catalog.json"),
            None,
            0
        )
        .await
        .expect_err("HTTP failure must be propagated")
        .to_string();
        assert!(error.contains("catalog.json"), "{error}");
        assert!(error.contains("404"), "{error}");
        Ok(())
    }

    fn launcher_file(hash: &str, size: u64) -> ExpectedFile {
        ExpectedFile {
            hash: Some(hash.into()),
            hash_type: Some(HashType::Crc64Xz),
            size: Some(size)
        }
    }

    #[tokio::test]
    async fn launcher_file_replaces_existing_copy_with_wrong_contents() -> Result<(), Box<dyn Error>>
    {
        let fixture = Fixture::new().await?;
        let output = fixture.directory.join("resources.assets");
        fs::write(&output, b"evil")?;
        let url = format!("{}/ok.bin", fixture.base_url);
        download_expected_file(&url, &output, &launcher_file(GOOD_CRC64, 4), 0).await?;
        assert_eq!(fs::read(&output)?, b"good");
        Ok(())
    }

    #[tokio::test]
    async fn launcher_file_rejects_wrong_checksum_or_size() -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new().await?;
        let output = fixture.directory.join("resources.assets");
        let url = format!("{}/ok.bin", fixture.base_url);
        for expected in [launcher_file("1", 4), launcher_file(GOOD_CRC64, 5)] {
            download_expected_file(&url, &output, &expected, 0)
                .await
                .expect_err("Mismatched launcher file must not be accepted");
            assert!(!output.exists());
        }
        Ok(())
    }
}
