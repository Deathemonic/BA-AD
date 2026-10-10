#![cfg(any(target_os = "linux", target_os = "macos"))]

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::path::PathBuf;
    use std::process::{self, Command, Output};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::{env, fs, io};

    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct DataDirectory(PathBuf);

    impl DataDirectory {
        fn new() -> io::Result<Self> {
            let directory = env::temp_dir().join(format!(
                "baad-update-test-{}-{}",
                process::id(),
                NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&directory)?;
            Ok(Self(directory))
        }

        fn run(&self) -> io::Result<Output> {
            Command::new(env!("CARGO_BIN_EXE_baad"))
                .arg("--update")
                .env("XDG_DATA_HOME", &self.0)
                .output()
        }
    }

    impl Drop for DataDirectory {
        fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
    }

    #[test]
    fn standalone_update_invalidates_metadata_and_preserves_cached_resources()
    -> Result<(), Box<dyn Error>> {
        let directory = DataDirectory::new()?;
        let cache = directory.0.join("baad");
        for (name, data) in [
            ("api_data.json", b"old API metadata".as_slice()),
            ("catalog/china/MediaManifest.txt", b"old media manifest"),
            ("catalog/china/MediaManifest.v2.bytes", b"old parsed catalog"),
            ("catalog/global/catalog.json", b"old global catalog"),
            ("apk/BlueArchiveGlobal.xapk", b"cached APK"),
            ("data/resources.assets", b"cached assets"),
            ("output/audio.ogg", b"downloaded audio")
        ] {
            let path = cache.join(name);
            fs::create_dir_all(path.parent().expect("Fixture file has a parent"))?;
            fs::write(path, data)?;
        }
        let output = directory.run()?;
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        assert!(!cache.join("api_data.json").exists());
        assert!(!cache.join("catalog").exists());
        assert_eq!(fs::read(cache.join("apk/BlueArchiveGlobal.xapk"))?, b"cached APK");
        assert_eq!(fs::read(cache.join("data/resources.assets"))?, b"cached assets");
        assert_eq!(fs::read(cache.join("output/audio.ogg"))?, b"downloaded audio");
        assert!(directory.run()?.status.success(), "Repeated updates must be safe");
        Ok(())
    }

    #[test]
    fn update_with_no_cached_metadata_succeeds() -> Result<(), Box<dyn Error>> {
        let directory = DataDirectory::new()?;
        let output = directory.run()?;
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        Ok(())
    }

    #[test]
    fn update_is_applied_before_a_download_command() -> Result<(), Box<dyn Error>> {
        let directory = DataDirectory::new()?;
        let cache = directory.0.join("baad");
        fs::create_dir_all(cache.join("catalog/china"))?;
        fs::write(cache.join("api_data.json"), b"old API metadata")?;
        // Reject the filter before any CDN request, after processing --update.
        let output = Command::new(env!("CARGO_BIN_EXE_baad"))
            .args([
                "--update",
                "download",
                "china",
                "--media",
                "--filter",
                "(",
                "--filter-method",
                "regex"
            ])
            .env("XDG_DATA_HOME", &directory.0)
            .output()?;
        assert_eq!(output.status.code(), Some(1), "The invalid filter must fail at runtime");
        assert!(!cache.join("api_data.json").exists());
        assert!(!cache.join("catalog").exists());
        Ok(())
    }
}
