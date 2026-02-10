# AGENTS.md - BA-AD Project Guide

This document helps AI agents work effectively in the BA-AD (Blue Archive Asset Downloader) codebase.

## Project Overview

BA-AD is a Rust workspace project that downloads assets from the Blue Archive mobile game. It consists of a CLI tool and a library for programmatic use.

**Repository**: https://github.com/Deathemonic/BA-AD  
**License**: Not specified (respect game copyrights)

## Workspace Structure

```
crates/
├── baad-cli/      # CLI binary (binary name: `baad`)
├── baad/          # Main library (asset downloading, catalogs, CDN)
├── baad-core/     # Core types, config, errors
├── baad-utils/    # Utilities (logging, file I/O, JSON, network)
├── baad-dm/       # Download manager (HTTP client, chunking, ZIP)
└── baad-apk/      # APK handling (fetch, extract)
```

## Essential Commands

### Build
```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo build -p baad-cli        # Build only CLI
```

### Run
```bash
cargo run --bin baad -- download japan --assets
cargo run --bin baad -- download global --media --filter "ch0230"
```

### Test
```bash
cargo test                     # Run all tests
cargo test -p baad             # Test specific crate
```

### Format
```bash
cargo +nightly fmt             # Requires nightly due to unstable_features in .rustfmt.toml
```

### Check/Lint
```bash
cargo check
cargo clippy
```

### Install
```bash
cargo install --path crates/baad-cli --locked --release
```

## Code Quality Principles

Follow **SOLID** (minus Liskov - SOID), **DRY**, and **KISS** principles:

- **No code smells** - Keep code clean and maintainable
- **No comments** - Code should be self-documenting via meaningful names
- **Self-documenting names** - No abbreviations unless universally known (e.g., `config` not `cfg`)
- **Fail fast** - Validate inputs early, avoid deep nesting (max 3-4 levels)
- **No magic numbers/strings** - Use named constants
- **Prefer composition over inheritance**
- **Keep methods small** - Ideally under 20 lines, single-purpose
- **Minimize dependencies** - Depend on abstractions, not concretions
- **No circular dependencies**
- **Prefer pure functions** where possible
- **Prefer immutability by default**

## Rust-Specific Guidelines

- **Leverage ownership** - Avoid unnecessary cloning, use references where possible
- **Use iterators** - Prefer iterator chains over manual loops
- **No `unsafe`** - Unless absolutely necessary and well-documented why
- **Use `Result`/`Option`** - Never panic for expected error cases
- **Type safety** - Leverage the type system, avoid primitive obsession

## Code Style & Conventions

### Rustfmt Configuration
Located in `.rustfmt.toml`. **Must use nightly toolchain** for formatting due to `unstable_features = true`.

Key settings:
- `edition = "2024"`
- `max_width = 100`
- `trailing_comma = "Never"`
- `group_imports = "StdExternalCrate"`
- `imports_granularity = "Module"`
- `fn_single_line = true`

### Naming Conventions
- **Types**: PascalCase (`DownloadConfig`, `ServerRegion`)
- **Functions/Variables**: snake_case (`download_assets`, `config_path`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Modules**: snake_case

### Import Style
Group imports: Std → External → Crate (enforced by rustfmt):
```rust
use std::path::PathBuf;

use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::config::ServerConfig;
```

### Error Handling
- Uses `eyre` for CLI error reporting
- Uses `thiserror` for library error definitions
- Custom error types in `baad-core/src/error.rs`
- Pattern: `Result<T, ErrorType>` or `eyre::Result<T>`
- **Never use panics for control flow** - Only for truly exceptional, unrecoverable cases
- **Fail fast** - Validate inputs at function entry, return errors early

### Async Patterns
- Uses `tokio` runtime
- Main entry: `#[tokio::main]`
- Async traits use `async_trait` (if needed)
- Prefer `?` for error propagation
- **Be mindful of allocations** in hot paths - avoid unnecessary clones in async contexts

## Key Dependencies

### Workspace Dependencies (defined in root Cargo.toml)
- `tokio = "1.47"` - Async runtime
- `eyre = "0.6"` - Error handling
- `thiserror = "2.0"` - Error derive macros
- `serde = "1.0"` - Serialization
- `serde_json = "1.0"` - JSON handling
- `reqwest = "0.12"` - HTTP client
- `bacy` (git) - Blue Archive cryptography library

### Crate-Specific
- `clap = "4.5"` - CLI parsing (baad-cli)
- `memorypack = "1.1"` - Binary serialization (baad)
- `zip = "5.1"` - ZIP handling (baad-dm)
- `nucleo = "0.5"` - Fuzzy matching (baad)
- `tracing-indicatif = "0.3"` - Progress bars via tracing spans (baad-utils, baad-dm)

## Architecture Patterns

### Module Structure
Each crate follows this pattern:
```rust
// lib.rs
pub mod submodule;
pub use submodule::{Type, function};

// submodule/mod.rs
mod internal;
pub use internal::PublicType;
```

### Type Definitions
- Core types in `baad-core/src/types.rs`
- Config in `baad-core/src/config.rs`
- Errors in `baad-core/src/error.rs`

### CLI Structure
Uses `clap` derive macros:
```rust
#[derive(Parser)]
#[command(name = "baad")]
pub struct Args { ... }

#[derive(Subcommand)]
pub enum Commands { ... }
```

### Server Configuration
- `ServerRegion::Japan` or `ServerRegion::Global`
- `Platform::Android` (default) or `Platform::Ios`
- `BuildType::Standard` (default) or `BuildType::Teen` (Global only)

## Testing Approach

- Unit tests inline in source files
- Integration tests in `tests/` directories (if present)
- No explicit test framework beyond `cargo test`

### Progress Bar Testing
Run the progress bar example:
```bash
cargo run --example test_download -p baad-dm
```
This simulates downloads to verify the Buck2-style progress display works correctly.

## CI/CD

GitHub Actions workflows in `.github/workflows/`:
- `build.yml` - Multi-platform release builds (Linux, Windows, macOS x86_64/aarch64)
- `build_core.yml` - Builds core library for external use

Triggers on version tags (`v*`) or releases.

## What to Avoid

- **God objects/classes** - Keep types focused and single-purpose
- **Primitive obsession** - Use strong types, not raw strings/ints
- **Feature envy** - Methods should operate on their own data
- **Dead code** - Remove unused code, don't leave commented-out blocks
- **Deep nesting** - Max 3-4 levels; use early returns
- **Empty error handlers** - Never ignore errors silently
- **Static dependencies** - Write testable code, use dependency injection
- **Premature optimization** - But don't write obviously inefficient code

## Progress Bars

Uses `tracing-indicatif` to show Buck2-style progress bars:

- Add `#[instrument]` to async functions to get a progress bar
- Progress bars show elapsed time and turn yellow (>5s) then red (>10s)
- After completion, normal logs appear (e.g., `info!(success = true, "Downloaded")`)
- Configure style in `baad-utils/src/progress.rs`

### Usage Example
```rust
use tracing::instrument;

#[instrument]
async fn download_file(name: &str) {
    // Progress bar shows during this work
    do_download().await;
    
    // Log appears after progress bar finishes
    info!(file = name, success = true, "Downloaded");
}
```

## Important Gotchas

1. **Rustfmt requires nightly**: Always use `cargo +nightly fmt`

2. **Edition 2024**: Project uses Rust 2024 edition (requires Rust 1.85+)

3. **Teen build restriction**: `BuildType::Teen` only works with `ServerRegion::Global`, not Japan

4. **Workspace dependencies**: Always add new dependencies to workspace root first, then reference with `{ workspace = true }`

5. **Binary name**: The CLI binary is named `baad` (not `baad-cli`), defined in `baad-cli/Cargo.toml`:
   ```toml
   [[bin]]
   name = "baad"
   path = "src/main.rs"
   ```

6. **External dependency**: Uses `bacy` from git (BA-CY cryptography library). Changes there affect this project.

7. **MemoryPack**: Uses `memorypack` crate for binary serialization of game catalog data

8. **Progress bars**: Logging must be initialized via `init_logging()` for progress bars to work. Uses `tracing-indicatif` internally.

## File Locations

- Main library entry: `crates/baad/src/lib.rs`
- CLI entry: `crates/baad-cli/src/main.rs`
- Core types: `crates/baad-core/src/types.rs`
- Core errors: `crates/baad-core/src/error.rs`
- CLI args: `crates/baad-cli/src/args.rs`
- Download logic: `crates/baad/src/download/`
- API clients: `crates/baad/src/api/`
- Progress bar styling: `crates/baad-utils/src/progress.rs`
- Progress example: `crates/baad-dm/examples/test_download.rs`

## Documentation

- Library usage: `.github/docs/LIBRARY.md`
- Changelog: `.github/docs/CHANGELOG.md`
- FAQ: `.github/docs/FAQ.md`

## Related Projects

- [BA-AX](https://github.com/Deathemonic/BA-AX): Asset extractor
- [BA-MU](https://github.com/Deathemonic/BA-MU): AssetBundle re-dumper
- [BA-FB](https://github.com/Deathemonic/BA-FB): FlatBuffer dumper
- [BA-CY](https://github.com/Deathemonic/BA-CY): Cryptography library
- [BA-BR](https://github.com/Deathemonic/BA-BR): AssetBundle repacker
