# baad

Core library for **Blue Archive - Asset Downloader**.

`baad` contains the game API clients, catalog fetchers, CDN resolvers, filtering logic, and resource download pipeline.
Use it when you want to integrate Blue Archive asset discovery and downloading directly into a Rust application.

## Install

```toml
[dependencies]
baad = { git = "https://github.com/Deathemonic/BA-AD" }
eyre = "0.6"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

By default, `baad` enables logging and progress helpers through the `logs` feature. Disable default features if you only
need the downloader core:

```toml
[dependencies]
baad = { git = "https://github.com/Deathemonic/BA-AD", default-features = false }
```

## Usage

Download all asset bundles from the Japan server into `./output`:

```rust
use baad::catalog::{Catalog, JapanCatalog};
use baad::download::{ResourceCategory, ResourceDownloader};
use baad::Platform;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let catalog = JapanCatalog::new(vec![ResourceCategory::Assets], Platform::Android)?;
    let downloads = catalog.prepare_downloads().await?;

    let downloader = ResourceDownloader::builder()
        .output_dir("./output".into())
        .limit(10)
        .retries(10)
        .build();

    downloader.download(downloads, None).await?;

    Ok(())
}
```

Prepare downloads without starting them:

```rust
use baad::catalog::{Catalog, GlobalCatalog};
use baad::download::ResourceCategory;
use baad::Platform;

#[tokio::main]
async fn main() -> baad::Result<()> {
    let catalog = GlobalCatalog::new(vec![ResourceCategory::Media], Platform::Android, false)?;
    let downloads = catalog.prepare_downloads().await?;

    println!("prepared {} media resources", downloads.len());

    Ok(())
}
```

Filter prepared resources before downloading:

```rust
use baad::download::{FilterMethod, ResourceFilter};

fn filter_resources(downloads: Vec<baad::download::ResourceDownload>) -> baad::Result<()> {
    let filter = ResourceFilter::new("ch0230", FilterMethod::ContainsIgnoreCase)?;
    let matching = filter.apply(downloads);

    println!("matched {} resources", matching.len());

    Ok(())
}
```

## Related Crates

- [`baad-cli`](../baad-cli): command-line interface built on top of this crate.
- [`baad-dm`](../baad-dm): lower-level download manager used by the resource downloader.
- [`baad-shared`](../baad-shared): shared types, constants, clients, and observer traits.
- [`baad-utils`](../baad-utils): utility helpers for files, JSON, networking, logging, and progress.

---

<sub>**Copyright** - Blue Archive is a registered trademark of NAT GAMES Co., Ltd., NEXON Korea Corp., and Yostar, Inc.
This project is not affiliated with, endorsed by, or connected to NAT GAMES Co., Ltd., NEXON Korea Corp., NEXON GAMES
Co., Ltd., IODivision, Yostar, Inc., or any of their subsidiaries or affiliates. All game assets, content, and materials
are copyrighted by their respective owners and are used for informational and educational purposes only.</sub>
