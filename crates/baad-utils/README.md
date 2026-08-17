# baad-utils

Utility crate for **Blue Archive - Asset Downloader**.

`baad-utils` contains reusable helpers for file I/O, JSON loading/saving, network helpers, logging setup, terminal formatting, progress rendering, and async runner utilities used by the rest of the workspace.

## Install

```toml
[dependencies]
baad-utils = { git = "https://github.com/Deathemonic/BA-AD" }
eyre = "0.6"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde = { version = "1", features = ["derive"] }
```

By default, `baad-utils` enables the `logs`, `observer`, and `utils` features. Disable default features for only file, JSON, and network helpers:

```toml
[dependencies]
baad-utils = { git = "https://github.com/Deathemonic/BA-AD", default-features = false, features = ["utils"] }
```

If you only need the logging stack, use `only_logging`. This drops `baad-shared`, `reqwest`, `tokio`, `serde`, `serde_json`, and `platform-dirs`:

```toml
[dependencies]
baad-utils = { git = "https://github.com/Deathemonic/BA-AD", default-features = false, features = ["only_logging"] }
```

## Usage

Save and load JSON with parent-directory creation handled for you:

```rust
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
struct CacheState {
    version: String,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let path = Path::new("./cache/state.json");

    baad_utils::json::save(
        path,
        &CacheState {
            version: "latest".into(),
        },
    )
    .await?;

    let state: CacheState = baad_utils::json::load(path).await?;
    println!("loaded version: {}", state.version);

    Ok(())
}
```

Create an output directory and save bytes:

```rust
use std::path::Path;

#[tokio::main]
async fn main() -> baad_utils::Result<()> {
    let output = baad_utils::file::get_output_dir(Some(Path::new("./output"))).await?;
    let file = output.join("hello.txt");

    baad_utils::file::save_file(&file, b"hello").await?;

    Ok(())
}
```

Update a JSON file in place:

```rust
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
struct State {
    runs: u32,
}

#[tokio::main]
async fn main() -> baad_utils::Result<()> {
    baad_utils::json::update::<State, _>(Path::new("./cache/state.json"), |state| {
        state.runs += 1;
    })
    .await?;

    Ok(())
}
```

## Feature Flags

- `logs` (default): tracing setup, console formatting, progress rendering, and the runner helpers.
- `observer` (default): download progress model wired into the `baad-shared` observer. Implies `logs`.
- `utils` (default): file I/O, JSON, and network helpers.
- `only_logging`: alias for `logs` alone. Pair with `default-features = false` to skip `observer` and `utils` and their dependencies.

With `only_logging`, `init_logging` still works but renders without the download progress view, since progress events come from the `observer` feature.

---

<sub>**Copyright** - Blue Archive is a registered trademark of NAT GAMES Co., Ltd., NEXON Korea Corp., and Yostar, Inc. This project is not affiliated with, endorsed by, or connected to NAT GAMES Co., Ltd., NEXON Korea Corp., NEXON GAMES Co., Ltd., IODivision, Yostar, Inc., or any of their subsidiaries or affiliates. All game assets, content, and materials are copyrighted by their respective owners and are used for informational and educational purposes only.</sub>
