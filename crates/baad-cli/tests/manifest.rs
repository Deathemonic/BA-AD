#![cfg(any(target_os = "linux", target_os = "macos"))]

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::path::{Path, PathBuf};
    use std::process::{self, Command, Output};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::{env, fs, io, thread};

    const GOOD_MD5: &str = "755f85c2723bb39381c7379a604160d8";
    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    /// Serves `good` at `/good.bin` and 404 for every other path.
    struct Fixture {
        directory: PathBuf,
        base_url: String
    }

    impl Fixture {
        fn new() -> io::Result<Self> {
            let directory = env::temp_dir().join(format!(
                "baad-cli-manifest-{}-{}",
                process::id(),
                NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&directory)?;
            let listener = TcpListener::bind("127.0.0.1:0")?;
            let base_url = format!("http://{}", listener.local_addr()?);
            thread::spawn(move || {
                for stream in listener.incoming().flatten() {
                    let _ = respond(stream);
                }
            });
            Ok(Self { directory, base_url })
        }

        fn entry(&self, path: &str, name: &str) -> String {
            format!(
                r#"{{"kind":"media","path":"{path}","url":"{}/{name}","size":4,"hash":"{GOOD_MD5}","hash_type":"md5"}}"#,
                self.base_url
            )
        }

        fn manifest(&self, region: &str, entries: &[String]) -> io::Result<PathBuf> {
            let path = self.directory.join("manifest.json");
            fs::write(
                &path,
                format!(
                    r#"{{"schema":1,"region":"{region}","platform":"android","build":"standard","version":null,"source":"{}","resources":[{}]}}"#,
                    self.base_url,
                    entries.join(",")
                )
            )?;
            Ok(path)
        }

        fn output(&self) -> PathBuf { self.directory.join("output") }

        fn report(&self) -> PathBuf { self.directory.join("report.json") }

        fn download(&self, region: &str, manifest: &Path, extra: &[&str]) -> io::Result<Output> {
            Command::new(env!("CARGO_BIN_EXE_baad"))
                .args(["download", region, "--manifest"])
                .arg(manifest)
                .arg("--output")
                .arg(self.output())
                .arg("--report")
                .arg(self.report())
                .args(["--retries", "0"])
                .args(extra)
                .env("XDG_DATA_HOME", self.directory.join("xdg"))
                .output()
        }

        fn report_json(&self) -> io::Result<String> { fs::read_to_string(self.report()) }
    }

    impl Drop for Fixture {
        fn drop(&mut self) { let _ = fs::remove_dir_all(&self.directory); }
    }

    fn respond(mut stream: TcpStream) -> io::Result<()> {
        let mut request = Vec::new();
        let mut buffer = [0_u8; 1024];
        while !request.windows(4).any(|bytes| bytes == b"\r\n\r\n") {
            let size = stream.read(&mut buffer)?;
            if size == 0 || request.len() > 8192 {
                return Ok(());
            }
            request.extend_from_slice(&buffer[..size]);
        }
        let request = String::from_utf8_lossy(&request);
        let found = request.split_whitespace().nth(1) == Some("/good.bin");
        let status = if found { "200 OK" } else { "404 Not Found" };
        let headers =
            format!("HTTP/1.1 {status}\r\nContent-Length: 4\r\nConnection: close\r\n\r\n");
        stream.write_all(headers.as_bytes())?;
        if !request.starts_with("HEAD ") {
            stream.write_all(b"good")?;
        }
        Ok(())
    }

    fn stderr(output: &Output) -> String {
        format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    }

    #[test]
    fn downloads_manifest_entries_and_reports_each_file() -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new()?;
        let manifest = fixture.manifest("japan", &[fixture.entry("Audio/a.ogg", "good.bin")])?;

        let output = fixture.download("japan", &manifest, &[])?;
        assert!(output.status.success(), "{}", stderr(&output));
        assert_eq!(fs::read(fixture.output().join("Audio/a.ogg"))?, b"good");
        let report = fixture.report_json()?;
        assert!(report.contains(r#""path": "Audio/a.ogg""#), "{report}");
        assert!(report.contains(r#""status": "downloaded""#), "{report}");

        let output = fixture.download("japan", &manifest, &[])?;
        assert!(output.status.success(), "{}", stderr(&output));
        assert!(fixture.report_json()?.contains(r#""status": "skipped""#));
        Ok(())
    }

    #[test]
    fn failed_files_are_reported_and_fail_the_command() -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new()?;
        let manifest = fixture.manifest("global", &[
            fixture.entry("a.ogg", "good.bin"),
            fixture.entry("missing.ogg", "missing.bin")
        ])?;

        let output = fixture.download("global", &manifest, &[])?;
        assert!(!output.status.success());
        assert_eq!(fs::read(fixture.output().join("a.ogg"))?, b"good");
        assert!(!fixture.output().join("missing.ogg").exists());
        let report = fixture.report_json()?;
        assert!(report.contains(r#""status": "downloaded""#), "{report}");
        assert!(report.contains(r#""status": "failed""#), "{report}");
        Ok(())
    }

    #[test]
    fn invalid_manifests_download_nothing() -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new()?;
        let manifest = fixture.manifest("japan", &[
            fixture.entry("a.ogg", "good.bin"),
            fixture.entry("../escape.ogg", "good.bin")
        ])?;
        let output = fixture.download("japan", &manifest, &[])?;
        assert!(!output.status.success());
        assert!(stderr(&output).contains("Unsafe output path"), "{}", stderr(&output));

        let manifest = fixture.manifest("japan", &[fixture.entry("a.ogg", "good.bin")])?;
        let output = fixture.download("china", &manifest, &[])?;
        assert!(!output.status.success());
        assert!(stderr(&output).contains("not china"), "{}", stderr(&output));

        assert!(!fixture.output().join("a.ogg").exists());
        assert!(!fixture.directory.join("escape.ogg").exists());
        assert!(!fixture.report().exists());
        Ok(())
    }

    #[test]
    fn manifest_cannot_be_combined_with_catalog_selection() -> Result<(), Box<dyn Error>> {
        let fixture = Fixture::new()?;
        let manifest = fixture.manifest("japan", &[])?;
        for extra in [["--filter", "a"], ["--platform", "ios"]] {
            let output = fixture.download("japan", &manifest, &extra)?;
            assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
        }
        Ok(())
    }
}
