#![cfg(any(target_os = "linux", target_os = "macos"))]

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::path::{Path, PathBuf};
    use std::process::{self, Command, Output};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::{env, fs, io};

    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TempDirectory(PathBuf);

    impl TempDirectory {
        fn new() -> io::Result<Self> {
            let directory = env::temp_dir().join(format!(
                "baad-data-dir-test-{}-{}",
                process::id(),
                NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&directory)?;
            Ok(Self(directory))
        }

        fn run(&self, working_dir: &Path, args: &[&str]) -> io::Result<Output> {
            Command::new(env!("CARGO_BIN_EXE_baad"))
                .args(args)
                .current_dir(working_dir)
                .env("XDG_DATA_HOME", self.0.join("xdg"))
                .output()
        }
    }

    impl Drop for TempDirectory {
        fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
    }

    fn write(path: &Path, data: &[u8]) -> io::Result<()> {
        fs::create_dir_all(path.parent().expect("Fixture file has a parent"))?;
        fs::write(path, data)
    }

    #[test]
    fn clean_uses_the_custom_data_directory() -> Result<(), Box<dyn Error>> {
        let directory = TempDirectory::new()?;
        let custom = directory.0.join("custom");
        let default = directory.0.join("xdg/baad");
        write(&custom.join("api_data.json"), b"custom metadata")?;
        write(&default.join("api_data.json"), b"default metadata")?;

        let output = directory.run(&directory.0, &[
            "--data-dir",
            custom.to_str().expect("UTF-8 path"),
            "--clean"
        ])?;

        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        assert!(custom.is_dir());
        assert!(!custom.join("api_data.json").exists());
        assert_eq!(fs::read(default.join("api_data.json"))?, b"default metadata");
        Ok(())
    }

    #[test]
    fn clean_refuses_a_data_directory_containing_the_working_directory()
    -> Result<(), Box<dyn Error>> {
        let directory = TempDirectory::new()?;
        let working_dir = directory.0.join("project/work");
        write(&working_dir.join("keep.txt"), b"user file")?;

        let output = directory.run(&working_dir, &["--data-dir", "..", "--clean"])?;

        assert!(!output.status.success());
        assert_eq!(fs::read(working_dir.join("keep.txt"))?, b"user file");
        Ok(())
    }
}
