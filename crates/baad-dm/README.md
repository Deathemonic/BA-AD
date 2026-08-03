# baad-dm

Download manager crate for **Blue Archive - Asset Downloader**.

`baad-dm` provides the lower-level download engine used by `baad`. It handles HTTP client setup, concurrent downloads, resumable files, chunked downloads, hash verification, summaries, progress events, and ZIP extraction from cached archives.

## Install

```toml
[dependencies]
baad-dm = { git = "https://github.com/Deathemonic/BA-AD" }
eyre = "0.6"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Usage

Download a list of URLs into `./output`:

```rust
use std::path::Path;

use baad_dm::{Download, Downloader, DownloaderConfig};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let downloads = vec![
        Download::try_from("https://example.com/archive.zip")?,
    ];

    let config = DownloaderConfig::builder()
        .directory(Path::new("./output"))
        .concurrent_downloads(8)
        .retries(3)
        .resumable(true)
        .build();

    let summaries = Downloader::new(config).download(&downloads).await;

    for summary in summaries {
        println!("{}: {:?}", summary.filename, summary.status);
    }

    Ok(())
}
```

Build a download from a URL:

```rust
use baad_dm::Download;

fn main() -> Result<(), baad_dm::Error> {
    let download = Download::try_from("https://example.com/file.bin")?;

    println!("save as {}", download.filename);

    Ok(())
}
```

Tune downloader behavior:

```rust
use std::path::Path;

use baad_dm::DownloaderConfig;

let config = DownloaderConfig::builder()
    .directory(Path::new("./output"))
    .concurrent_downloads(4)
    .max_chunks_per_file(8)
    .overwrite(false)
    .build();
```

---

<sub>**Copyright** - Blue Archive is a registered trademark of NAT GAMES Co., Ltd., NEXON Korea Corp., and Yostar, Inc. This project is not affiliated with, endorsed by, or connected to NAT GAMES Co., Ltd., NEXON Korea Corp., NEXON GAMES Co., Ltd., IODivision, Yostar, Inc., or any of their subsidiaries or affiliates. All game assets, content, and materials are copyrighted by their respective owners and are used for informational and educational purposes only.</sub>
