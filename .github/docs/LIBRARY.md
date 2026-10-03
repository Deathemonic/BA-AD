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

### Logging Without BA-AD

If you only want the log formatting and terminal output no catalogs, downloads, or file
helpers, depend on `baad-utils` directly with the `only_logging` feature. It pulls in the
tracing stack alone, dropping `baad-shared`, `reqwest`, `tokio`, `serde`, `serde_json`, and
`platform-dirs`:

```toml
[dependencies]
baad-utils = { git = "https://github.com/Deathemonic/BA-AD", default-features = false, features = ["only_logging"] }
```

```rust
use baad_utils::config::{init_logging, LoggingConfig};
use baad_utils::info;

init_logging(LoggingConfig::default())?;
info!(success = true, "logging ready");
```

`init_logging`, `flush_logs`, `run`, `run_async`, [`formatter`](../../crates/baad-utils/src/formatter/mod.rs),
and [`progress`](../../crates/baad-utils/src/progress/mod.rs) remain available. The download
progress view is not wired up, since that requires the `observer` feature and its
`baad-shared` dependency; `enable_progress` is ignored and logs render as plain lines.

## Custom Progress Display

To replace the built-in progress lines with your own rendering (e.g., progress bars),
implement [`ProgressModel`](../../crates/baad-utils/src/progress/view.rs) and
[`ProgressHandler`](../../crates/baad-utils/src/progress/model.rs), then initialize
logging with `init_logging_with_model` instead of `init_logging`:

```rust
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::Arc;

use baad::{
    init_logging_with_model, LoggingConfig, ProgressEvent, ProgressHandler, ProgressModel,
};

#[derive(Default)]
struct BarModel {
    active: HashMap<Arc<str>, (u64, u64)>, // id -> (current, total)
}

impl ProgressModel for BarModel {
    // Called on repaint. Write one line per active task; `width` and `height` are
    // the terminal dimensions. Keep line count under `height - 1` — drawing more lines
    // than the terminal has rows corrupts the repaint.
    fn render(&mut self, width: usize, height: usize, out: &mut String) {
        for (id, (current, total)) in &self.active {
            let pct = *current as f64 / (*total).max(1) as f64;
            let bar_width = width.saturating_sub(40).max(10);
            let filled = (bar_width as f64 * pct) as usize;
            let _ = writeln!(
                out,
                "{id} [{}{}] {:.0}%",
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

impl ProgressHandler for BarModel {
    // Called for every progress event. Update your state here.
    fn handle_event(&mut self, event: ProgressEvent) {
        match event {
            ProgressEvent::Started { id, total, .. } => {
                self.active.insert(id, (0, total));
            }
            ProgressEvent::Advance { id, current, total } => {
                self.active.insert(id, (current, total));
            }
            ProgressEvent::Completed { id, .. } => {
                self.active.remove(&id);
            }
        }
    }
}

init_logging_with_model(LoggingConfig::default(), BarModel::default())?;
```

Each `Started` event also carries a `label` (`&'static str`, e.g. `"Downloading"`) and a
`unit` ([`ProgressUnit::Bytes`](../../crates/baad-shared/src/observer.rs) or `Count`) so a
model can render byte sizes or plain counts and show the active verb per task.

The rendering machinery (repainting, cursor handling, interleaving log lines with the
progress area) is handled for you. If the progress display cannot be used (non-terminal
output, `enable_json`, `enable_progress: false`), the model is ignored and logging falls
back to plain output.

## Progress (Observers)

For full control — routing events to a GUI, a channel, or your own rendering loop —
implement [`ProgressObserver`](../../crates/baad-shared/src/observer.rs) and register it
globally with `set_observer` before downloading. This is the low-level hook underneath
the progress display; if you use it, disable the built-in progress with
`enable_progress: false` so the two don't compete for the observer slot.

```rust
use std::sync::Arc;
use baad::{set_observer, ProgressEvent, ProgressObserver};

struct MyObserver;

impl ProgressObserver for MyObserver {
    fn on_event(&self, event: ProgressEvent) {
        match event {
            ProgressEvent::Started { id, label, total, .. } => {
                println!("{label} {id} ({total} total)");
            }
            ProgressEvent::Advance { id, current, total } => {
                println!("{id}: {current}/{total}");
            }
            ProgressEvent::Completed { id, status } => {
                println!("done {id}: {status:?}");
            }
        }
    }
}

set_observer(Arc::new(MyObserver));
```

`set_observer` can only be called once per process; the first caller wins. If no observer
is set, a `NoopObserver` is used and events are discarded.

### Emitting Your Own Progress

The event vocabulary is generic, so you can drive the same display from your own work, not
just downloads. Use the [`Progress`](../../crates/baad-shared/src/observer.rs) guard:
it emits `Started` on creation, `Advance` on each `advance`, and exactly one `Completed`
on `finish`/`fail` or when dropped (defaulting to success).

```rust
use baad::{Progress, ProgressUnit};

let progress = Progress::start("Excel.zip", "Extracting", ProgressUnit::Bytes, total_bytes);
for chunk in chunks {
    write_chunk(chunk)?;
    progress.advance(chunk.len() as u64);
}
progress.finish();
```

Pass `ProgressUnit::Count` when the numbers are item counts (e.g. tables processed) rather
than byte sizes, and the display renders `3 / 10` instead of `3 B / 10 B`.

---

See [parse.rs](../../crates/baad-cli/src/parse.rs) for a complete reference on how the CLI uses the API.

## C API

BAAD ships one native shared library (`libbaad.so`, `libbaad.dylib`, or `baad.dll`)
with matching Diplomat-generated C headers. Include the owning API family
header (`baad/bindings/baad.h`, `baad-shared/bindings/baad_shared.h`,
`baad-dm/bindings/baad_dm.h`, or `baad-utils/bindings/baad_utils.h`) and link
that library. The same directory layout is preserved inside release packages. Package names remain `baad-ffi`,
`baad-dm-ffi`, `baad-shared-ffi`, and `baad-utils-ffi`. Among API packages, only `baad-ffi` builds a
shared library; each FFI crate defines its own API and is linked into it.
The `baad` family does not copy or rename shared, utility, or DM APIs. Native
Rust callers continue using the crates under `crates/`.

`Baad*` exposes catalogs, CDNs, clients, filters, strategies, and resource
operations. `BaadDm*` exposes downloads, summaries, hashes, ranges, and ZIP
operations. `BaadShared*` exposes platform/data types, HTTP initialization, and
observers. `BaadUtils*` exposes logging, files, JSON utilities, and terminal
helpers. All families use the same linked copy of the native global state.

### Build and package

From the repository root:

```sh
cargo build --locked --release -p baad-ffi
cargo run --locked -p baad-ffi-build -- --out target/baad-c
```

Each FFI crate has a Cargo build script. Shared Rust build support reads native
source declarations, generates the Diplomat bridge into Cargo's output directory,
and writes generated headers into that crate's `bindings/` directory. Native
enum variants, shared model fields/getters, scalar logging/download configuration
records, aligned-line records, resource-category flags, and public constants are generated from their defining Rust source.
Updating a supported native declaration updates its boundary declaration and
header on the next build. Native declarations remain the source of truth;
unsupported field or constant types require an explicit boundary adapter.

The bridge, runtime, and Rust header generator are pinned to Diplomat 0.14.0.
No separate Python scripts or generator installation are required. The Rust
packaging command builds the library and regenerates bindings before copying it and its matching per-crate
headers (including required runtime headers), with a SHA-256 manifest. For a
cross build, pass `--target` to the packaging command. Use `--profile debug` for
a development package. Packaging replaces an existing BAAD package's generated
contents to remove stale artifacts; other nonempty directories are rejected.

`baad-utils` also provides filename predicates, typed JSON values and updates,
proxy and HTTP response handles, line/URL formatting and style helpers, native
progress displays/views, and native runner adapters. Formatter/progress generics
are specialized to the native display model. Rust tracing subscriber visitors,
`MakeWriter`/`LoggingSink` trait implementations, and generic Rust futures remain
Rust abstractions, rather than copied public records. The native runner adapters
initialize logging and exit the process on logging or job failure, matching Rust.

### Ownership and errors

Fallible functions return generated result structs. Check `is_ok` before reading
the corresponding `ok` or `err` union member. Errors are owned `BaadError*` values:
read their `kind`, write their message, and call `BaadError_destroy` once.
Optional results also use `is_ok`, except optional opaque references which use
NULL. Valid C enum values are required.

Constructor/fetch results, owned key/name snapshots, buffers, and progress events
must be destroyed once with their generated type-specific `*_destroy` function.
Strings and primitive arrays returned as views borrow their input or handle;
copy them if needed beyond that owner's lifetime. Nested getters return borrowed
`const` handles without allocation or copying native fields. Never destroy,
mutate, or cast away const on those borrowed handles. Strategy input catalog
handles can also be created with `parse_json` when loading existing serialized
files; catalog workflows themselves exchange typed handles. Keep their owning root
alive and unmodified while using any borrowed view.

Strategy and download operations move data out of mutable handles, leaving them
empty. Append moves entries from its second handle. Those handles still require
destruction. Download validation errors leave inputs intact; after validation,
entries are moved even if execution subsequently fails or is rejected. Release borrowed views before these operations. Do not pass the same
handle as both arguments to append. Mutable inputs must have exclusive access.

Inputs use pointer/length views, without requiring NUL termination. Strings must
be valid UTF-8; `diplomat_is_str` can validate untrusted bytes before constructing
a string view. String arrays are validated by adapters and return an argument
error on invalid UTF-8. NULL with length zero is supported for empty slices;
nonempty slices require valid readable memory. Opaque handles and writers require
valid non-NULL pointers. Diplomat does not provide the previous null-pointer
status checks for invalid handles. Double destruction and stale handles violate
the ownership contract.

Writers can use `diplomat_simple_write` or the runtime's growing buffer functions.
Check `grow_failed` (or sufficient capacity) separately from an operation result:
a successful operation can have a truncated output when the writer cannot grow.
Destroy growing writers with `diplomat_buffer_write_destroy`.

### Initialization, blocking, and threads

Initialize logging before registering an observer. Native console progress can
install its own observer when enabled; set `enable_progress=false` when providing
a C observer. Set the data directory before constructing catalogs. Configure the
shared HTTP client before any shared-client request. Repeated client, logging,
app-name, or data-directory initialization reports an error. The shared HTTP
client config applies to catalogs/API requests and utility network requests.
Native download and ZIP operations construct their own HTTP clients using their
native configuration; explicit utility proxy requests also construct a client
for that proxy; downloader proxy/HTTP1 options remain per operation.

Catalog network/cache methods, CDN fetches, API requests, downloads, ZIP requests,
URL resolution, version fetching, async file operations, and JSON file operations
block the calling thread on one internal multithread Tokio runtime. Hash checking
performs synchronous disk I/O. Call these from application worker threads if your
UI/event loop must remain responsive. Calls from a Tokio runtime, progress
callback, or callback destructor are rejected with a Runtime error. The API does
not provide futures, completion callbacks, cancellation, or language async SDKs.
The internal runtime lives for the process lifetime; keep the BAAD library loaded
after using blocking operations. Runtime shutdown/unloading is not exposed.

Observer registration transfers ownership of callback context to the library.
Supply a destructor if the context needs releasing. The context and both callback
functions must remain usable until the destructor runs. Callbacks and destructors
may run on any operation/worker thread. Callback state must support concurrent
invocation (`Send + Sync` in Rust terms); synchronize application state as needed.
Callbacks must return normally and must not unwind, longjmp, or throw across the
boundary. Code containing their function pointers must remain loaded.

Each callback receives an owned `BaadSharedProgressEvent*`, even when no context
is supplied. Destroy every event, or retain it and destroy it later. Its views
remain valid until the event is destroyed. Event-kind, unit, and status-kind enums are generated from the corresponding
native enums. DM summaries use the same shared status-kind type.

Replacement and clear affect newly acquired observers. Existing progress objects
and download configurations retain their observer snapshot. Clear returns
without waiting for in-flight callbacks or active progress operations; it is not
a drain barrier. The old context destructor runs exactly once after the last
snapshot and callback are released. Clear/replace can be called inside callbacks
and destructors: no observer lock is held while invoking or dropping foreign
state. Do not wait in a callback for the blocking operation that invoked it. Do
not access the active operation's mutable handles from its callback. Logging,
event inspection, destruction of owned events, and observer clear/replace are
permitted reentrant operations. A reentrant registration made by a destructor
can supersede the registration that caused that destructor to run.

JSON update callbacks run only during that blocking call. Their mutable JSON
handle is borrowed for the duration of the callback: it must not be destroyed,
retained, or used after the callback returns. Filename-predicate string views
have the same call-scoped lifetime. Diplomat's callback-reference override is
limited by these contracts; retained progress callbacks still receive owned
objects. JSON updates preserve native behavior, including falling back to a
default value if loading the existing file fails.

Progress display handles are consumed by logging/model-view constructors and
remain destroyable. Views use the native progress model and mutex; a view can
receive an event, write a message, clear, and finish into an owned display. Finish
and destruction require exclusive access. Event submission clones the native
owned event so the original event remains usable. Response-body reads consume
the body while leaving the response handle destroyable.

Serialize access to opaque handles unless explicitly supported otherwise.
ResourceFilter contains a native RefCell-backed fuzzy matcher and must not be
matched concurrently. A BaadSharedProgress can be advanced from multiple threads
while its handle remains alive and no finish/fail/destroy is running; completion
and destruction require exclusive access. Other borrowed data is immutable while
its owner is unmodified.

### C API migration

This release intentionally breaks the previous handwritten C ABI. Consumers must
recompile against the matching generated headers and update their declarations.

| Previous interface | Replacement |
| --- | --- |
| Four independent shared libraries | One `baad` library containing all API families |
| Copied `baad_*` logging/client exports | `BaadUtils_*` logging and `BaadSharedClient_*` initialization |
| `baad_resource_filter_new/matches/free` | `BaadResourceFilter_new/matches/destroy` |
| Integer status and output parameters | Generated result unions with `is_ok`, owned errors with `kind/message/destroy` |
| NUL-terminated strings and allocated string outputs | UTF-8 pointer/length views or Diplomat writers |
| JSON catalog transport | Typed owning data handles, borrowed nested getters, slices, and map-key snapshots |
| Handwritten arrays/handles/free functions | Generated opaque handles and destroy functions |
| Strategy JSON inputs | Mutable typed catalog handles; contents are moved out and emptied |
| Downloads reused by cloning at the boundary | Download operations move entries out of mutable handles; handles remain destroyable |
| One-shot observer installation | Replace/clear registration with retained snapshots and context destruction |
| Borrowed callback records | Owned event handles, destroyed by the callback/consumer |
| Logging config pointer / optional proxy strings | By-value generated configuration; empty string selects absent proxy/hash/agent |
| Missing terminal size | Width and height zero |

Filter enum values retain the native ordering: Exact, Contains, Regex, Fuzzy,
Glob, ContainsIgnoreCase, StartsWith, EndsWith. Generated C enum names use the
corresponding `BaadFilterMethod_` prefix. Only declared enum values are valid.
Zero concurrency/chunk limits and invalid downloader proxies now report an error
before consuming input, preventing hangs or silent proxy fallback.
Optional scalar values such as download size and range end use generated
`OptionU64`; size zero remains distinct from an absent size.

Data getters expose native storage through transparent read-only wrappers.
Compile-time layout assertions cover those wrappers. Fetched data and key/name
snapshots are owned; nested field handles and returned strings/arrays are
borrowed. A borrowed field's generated destroy declaration does not grant
ownership: destroy only root objects returned as owned results.

`BaadGlobalStrategy_build_downloads` takes GlobalCatalogData and consumes its
resource list. Other catalog strategy methods consume catalog contents. Strategy
input catalog types also expose optional `parse_json` factories for existing
serialized files; returned data and inter-operation transport remain typed.
`BaadSharedGlobalCatalogData_new/push_resource` and
`BaadSharedDownloads_new/push_asset/push_table/push_media/append` support callers
constructing typed inputs without serialization. Use `BaadDmDownloads_new/push`
for direct DM jobs. Fields returned by getters are read-only; arbitrary editing
of the old mirror records is replaced by these deliberate input builders.

Download configuration defaults now come from native Rust builders. Resource
download concurrency and retries default to 10; concurrency uses native `usize`
(`size_t` in C). Progress event and status kinds are typed generated enums.

The C API is always built; the obsolete c-api/UniFFI feature selectors are removed.
Native async operations retain their blocking C behavior. Native error variants
remain unchanged; boundary error categories identify Filter, Catalog, Download,
File, Json, Network, Configuration, InvalidArgument, or Runtime. Catalog and
Download errors preserve their native display message, rather than exposing
binding-specific copies of each nested Rust error type. Native ResourceDownloader
continues reporting per-file failure through progress/logging while its overall
operation can succeed; use DM summaries when per-file outcomes are required.

UniFFI dependencies, configuration, build support, bindgen binaries, and release
jobs are removed. Generated Kotlin, Swift, Python, and Ruby SDKs are no longer
shipped. No BoltFFI pipeline is introduced. Languages using the C ABI must supply
their own declarations and ownership/error/thread adapters.

Native Rust changes are limited to explicit initialization failure reporting via
`client::try_set_client`, replaceable observers with lock-free foreign invocation,
owned `Arc<str>` progress labels, and cloneable owned progress events. Static labels still work with Progress::start;
code constructing Started events directly must use `label.into()`. This removes
the former binding adapter's permanently leaked label interning table. Native
Rust crates remain independent of Diplomat and other binding generators.
