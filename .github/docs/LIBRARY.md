# Using BA-AD as a Library

## Getting Started

Add `baad` to your `Cargo.toml`. You also need an async runtime (`tokio`) and an error type
(`eyre` is used here, but anything that `?` works with is fine):

```toml
[dependencies]
baad = { git = "https://github.com/Deathemonic/BA-AD" }
tokio = { version = "1", features = ["full"] }
eyre = "0.6"
```

## Quick Start

Download all asset bundles from the `JP` server into `./output`:

```rust
use baad::catalog::{Catalog, JapanCatalog};
use baad::download::{ResourceCategory, ResourceDownloader};
use baad::Platform;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Pick what to download and from which platform
    let catalog = JapanCatalog::new(ResourceCategory::Assets, Platform::Android)?;

    // Fetch the catalog and turn it into a list of downloads
    let downloads = catalog.prepare_downloads().await?;

    // Download into ./output
    let downloader = ResourceDownloader::builder()
        .output_dir("./output".into())
        .limit(10)
        .retries(10)
        .build();
    downloader.download(downloads, None).await?;

    Ok(())
}
```

The flow is always the same: **build a catalog → `prepare_downloads()` → feed it to a `ResourceDownloader`**.

## Data Directory

BA-AD stores cached API and catalog data in the platform data directory by default. To use a custom directory, set it once at startup before creating any catalogs:

```rust
use baad::file;

file::set_data_dir("./my-cache".into())?;

let catalog = JapanCatalog::new(ResourceCategory::Assets, Platform::Android)?;
```

`set_data_dir` is process-global and can only be called once. Use it for app-level cache configuration before constructing catalogs.

## Catalogs

Each server has its own catalog type, all implementing the [`Catalog`](../../crates/baad/src/catalog/traits.rs) trait (which provides
`prepare_downloads()`).

```rust
use baad::catalog::{ChinaCatalog, GlobalCatalog, JapanCatalog};
use baad::download::ResourceCategory;
use baad::{BuildType, Platform};

let categories = ResourceCategory::from([ResourceCategory::Assets, ResourceCategory::Media]);

// Japan
let japan = JapanCatalog::new(categories, Platform::Android)?;

// China
let china = ChinaCatalog::new(categories, Platform::Android)?;

// Global also takes a BuildType (Standard or Teen)
let global = GlobalCatalog::new(categories, Platform::Ios, BuildType::Teen)?;
```

> **Note:** `BuildType::Teen` is only valid for the Global server.

### ResourceCategory

What kind of files to pull:

```rust
use baad::download::ResourceCategory;

ResourceCategory::Assets   // asset bundles
ResourceCategory::Tables   // table bundles
ResourceCategory::Media    // media resources (audio, video, etc.)
ResourceCategory::ALL      // everything
```

Pass a single category directly:

```rust
let catalog = JapanCatalog::new(ResourceCategory::Assets, Platform::Android)?;
```

Or combine categories with `ResourceCategory::from`:

```rust
let categories = ResourceCategory::from([
    ResourceCategory::Assets,
    ResourceCategory::Media,
]);

let catalog = JapanCatalog::new(categories, Platform::Android)?;
```

You can also build categories conditionally:

```rust
let categories = ResourceCategory::new()
    .include_if(include_assets, ResourceCategory::Assets)
    .include_if(include_tables, ResourceCategory::Tables)
    .include_if(include_media, ResourceCategory::Media)
    .or_all_if_empty();
```

`ResourceCategory` is `Copy`, so you can reuse it across catalog constructors without cloning.

### Platform

```rust
use baad::Platform;

Platform::Android
Platform::Ios
Platform::Windows
```

## ResourceDownloader

Built with a builder. Only `output_dir`, `limit`, and `retries` are required; `proxy` is optional.

```rust
use baad::download::ResourceDownloader;

let downloader = ResourceDownloader::builder()
    .output_dir("./output".into())   // PathBuf
    .limit(10)                       // concurrent downloads
    .retries(10)                     // retry attempts per file
    .maybe_proxy(Some("http://127.0.0.1:8080".into())) // optional
    .build();

// Second arg is an optional filter (see below)
downloader.download(downloads, None).await?;
```

The `download` method takes the [`Downloads`](../../crates/baad-shared/src/types.rs) returned by `prepare_downloads()` and downloads
every category present in it.

## Filtering

To download only files whose path matches a pattern, pass a `ResourceFilter`:

```rust
use baad::download::{FilterMethod, ResourceFilter};

// Via constructor
let filter = ResourceFilter::new("ch0230", FilterMethod::Contains)?;

// Or the shorthand helpers
let filter = ResourceFilter::contains("ch0230")?;
let filter = ResourceFilter::regex(r"(ch0230|ch0255|hoshino).*battle")?;
let filter = ResourceFilter::fuzzy("ch0069")?;
let filter = ResourceFilter::glob("audio/voc_jp/**")?;

downloader.download(downloads, Some(&filter)).await?;
```

Available [`FilterMethod`](../../crates/baad/src/download/filter.rs) variants:

| Method                 | Matches when the path…                   |
|------------------------|------------------------------------------|
| `Exact`                | equals the pattern                       |
| `Contains`             | contains the pattern                     |
| `ContainsIgnoreCase`   | contains the pattern, case-insensitive   |
| `StartsWith`           | starts with the pattern                  |
| `EndsWith`             | ends with the pattern                    |
| `Regex`                | matches the regular expression           |
| `Fuzzy`                | fuzzy-matches the pattern                |
| `Glob`                 | matches the glob pattern                 |

Each method has a matching helper constructor (`ResourceFilter::exact`, `::starts_with`,
`::ends_with`, `::contains_ignore_case`, etc.).

## Downloads

`prepare_downloads()` returns a [`Downloads`](../../crates/baad-shared/src/types.rs) struct you can inspect before downloading:

```rust
let downloads = catalog.prepare_downloads().await?;

println!("assets: {}", downloads.assets.len());
println!("tables: {}", downloads.tables.len());
println!("media:  {}", downloads.media.len());
```

To list files without downloading them, iterate over the returned vectors:

```rust
use baad::catalog::{Catalog, JapanCatalog};
use baad::download::ResourceCategory;
use baad::Platform;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let catalog = JapanCatalog::new(
        ResourceCategory::from([ResourceCategory::Assets, ResourceCategory::Tables]),
        Platform::Android,
    )?;

    let downloads = catalog.prepare_downloads().await?;

    for asset in &downloads.assets {
        println!("asset: {} ({} bytes)", asset.path, asset.size);
    }

    for table in &downloads.tables {
        println!("table: {} ({} bytes)", table.path, table.size);
    }

    Ok(())
}
```

Each entry also includes the source `url`, expected `hash`, and, for assets/tables, any `bundle_files` listed by the catalog.

## Logging

Logging is off by default. Initialize it once at startup with `init_logging`:

```rust
use baad::{init_logging, LoggingConfig};

init_logging(LoggingConfig {
    enable_console: true,
    enable_debug: true,
    ..LoggingConfig::default()
})?;
```

When running in an interactive terminal, `init_logging` also renders a live progress display
for active downloads. To turn that off while keeping normal log output, set `enable_progress: false`:

```rust
init_logging(LoggingConfig {
    enable_progress: false,
    ..LoggingConfig::default()
})?;
```

The progress display is automatically disabled when output is not a terminal, when
`enable_json` is set, or when `enable_console` is off.

## Custom Progress Display

To replace the built-in `Downloading` lines with your own rendering (e.g., progress bars),
implement [`ProgressModel`](../../crates/baad-utils/src/progress/view.rs) and
[`DownloadProgressHandler`](../../crates/baad-utils/src/progress/model.rs), then initialize
logging with `init_logging_with_model` instead of `init_logging`:

```rust
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::Arc;

use baad::{
    init_logging_with_model, DownloadEvent, DownloadProgressHandler, LoggingConfig, ProgressModel,
};

#[derive(Default)]
struct BarModel {
    active: HashMap<Arc<str>, (u64, u64)>, // filename -> (downloaded, total)
}

impl ProgressModel for BarModel {
    // Called on repaint. Write one line per active download; `width` is the terminal width.
    fn render(&mut self, width: usize, out: &mut String) {
        for (name, (downloaded, total)) in &self.active {
            let pct = *downloaded as f64 / (*total).max(1) as f64;
            let bar_width = width.saturating_sub(40).max(10);
            let filled = (bar_width as f64 * pct) as usize;
            let _ = writeln!(
                out,
                "{name} [{}{}] {:.0}%",
                "█".repeat(filled),
                "░".repeat(bar_width - filled),
                pct * 100.0
            );
        }
    }

    // Called once when the display finishes. Optional summary output.
    fn final_message(&mut self, out: &mut String) {
        let _ = writeln!(out, "done");
    }
}

impl DownloadProgressHandler for BarModel {
    // Called for every download event. Update your state here.
    fn handle_event(&mut self, event: DownloadEvent) {
        match event {
            DownloadEvent::Started { filename, total_bytes } => {
                self.active.insert(filename, (0, total_bytes));
            }
            DownloadEvent::Progress { filename, downloaded_bytes, total_bytes } => {
                self.active.insert(filename, (downloaded_bytes, total_bytes));
            }
            DownloadEvent::Completed { filename, .. } => {
                self.active.remove(&filename);
            }
        }
    }
}

init_logging_with_model(LoggingConfig::default(), BarModel::default())?;
```

The rendering machinery (repainting, cursor handling, interleaving log lines with the
progress area) is handled for you. If the progress display cannot be used (non-terminal
output, `enable_json`, `enable_progress: false`), the model is ignored and logging falls
back to plain output.

## Progress (Observers)

For full control — routing events to a GUI, a channel, or your own rendering loop —
implement [`DownloadObserver`](../../crates/baad-shared/src/observer.rs) and register it
globally with `set_observer` before downloading. This is the low-level hook underneath
the progress display; if you use it, disable the built-in progress with
`enable_progress: false` so the two don't compete for the observer slot.

```rust
use std::sync::Arc;
use baad::{set_observer, DownloadEvent, DownloadObserver};

struct MyObserver;

impl DownloadObserver for MyObserver {
    fn on_event(&self, event: DownloadEvent) {
        match event {
            DownloadEvent::Started { filename, total_bytes } => {
                println!("start {filename} ({total_bytes} bytes)");
            }
            DownloadEvent::Progress { filename, downloaded_bytes, total_bytes } => {
                println!("{filename}: {downloaded_bytes}/{total_bytes}");
            }
            DownloadEvent::Completed { filename, status, .. } => {
                println!("done {filename}: {status:?}");
            }
        }
    }
}

set_observer(Arc::new(MyObserver));
```

`set_observer` can only be called once per process; the first caller wins. If no observer
is set, a `NoopObserver` is used and events are discarded.

---

See [parse.rs](../../crates/baad-cli/src/parse.rs) for a complete reference on how the CLI uses the API.
