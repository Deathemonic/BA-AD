# Changelogs

## 3.1.0

### Features

- Added `--boost` that accelerates downloading at the risk of cdn rate limiting

### API Changes

- Added `only_logging` feature on `baad-utils`
    - Adds `observer` and `utils` features
- More options on `ResourceDownloader`
    - You can now edit: `max_chunks_per_file`, `max_concurrent_chunks`, and `chunk_threshold`
    - You can now enable: `http1_only`
    - Output dir nows default to `./output` if not set
    - `limit` and `retries` now have 10 as default
- Increase the catalog and `resources.assets` fetching retries from 3 to 5
- `baad-dm` now accepts http1 downloads
    - Can be controlled by passing `http1_only` on `DownloaderConfig`
- Codebase cleanups

### Fixes

- Fixed catalog not redownloading when there's a hash mismatch

## 3.0.0

### Features

- No more APK dependencies, now uses the `resources.assets` assetbundle from the Yostar launcher api (reduced fetch size from 200MB down to 60MB)
- Added **China** assets downloading support
  - AssetBundle, TableBundle, and MediaResources downloading
  - Uses a `.hash` file to detect changes similar to the game
- Added back **Global** downloading
- Added Windows variant of MediaResources for Japan
- Added support to download TablePatchPacks
- Added FFI support, both C-ABI and UniFFI
- Output now categorizes what type of resource you are downloading
- Download logs now display the size progress and file name instead of the file path
- Added `--platform` to replace `--ios`, can now switch between `ios`, `android`, and `windows`
- Added two verbose options: minimal and full
- Improved zip extraction with CD caching
- Improved the catalog update and cache detection
- Improved allocations and overall performance across the codebase

### API Changes

- Merged `BA-AD-Core` and `Trauma` (fork) into one repository
- `baad-core` has been renamed to `baad-shared`
- `config.rs` has been split into `consts.rs` and `platform.rs`
- Combined the two `fetch_version` functions into `baad_utils::network`
- `ResourceCategory` no longer uses `vec!()`, now uses `ResourceCategory::from()` or `ResourceCategory::new()`
- Moved handling of categories to the catalog instead of the downloader
- Moved Windows-not-available handling from CLI level to library level
- Changed `WARN` to `WARNING` in logs
- Changed Global version check URL from PlayStore to Apptopia
- Progress log can now be disabled via API and via feature
- Allow injecting a custom progress model on the API
- Some functions are now const evaluated
- Some public API functions changed from borrowed to move to reduce cloning
- Bumped crate dependencies
- General codebase cleanup

### Fixes

- Fixed both CN TableBundles and MediaResources not downloading
- Fixed Global returning an error when fetching a catalog ([#16](https://github.com/Deathemonic/BA-AD/issues/16))
- Fixed Global always re-fetching catalogs on just a platform change
- Fixed switching platforms not reflecting on the next run
- Fixed the platform value on `api_data.json` not updating
- Fixed download logs using the file path instead of just the file name
- Fixed download logs going over the terminal height
- Fixed the `WARNING` log tag being misaligned
- Fixed CRC checking doing more allocations than needed
- Fixed the Arc increment hotspot
- Fixed URL format logging doing more allocations than needed
- Fixed logging messages not correctly displaying the message based on the action

## 2.9.2

### Fixes

- Fixed logging not outputing anything
- Global android not downloading anything


## 2.9.0

### Features

- Improve performance on
  - Extract - pre determine capacity
  - Fetching catalogs - directly deserialize
  - Looking game config - uses `memchr` and `rayon` with sorting to boost search

### API Changes

- Bump `BA-CY` to v2.5.5
  - Added new `memorypack` crate
  - Simplified `Media` and `Table` catalogs
  - Inlined some function
  - Improve performance
- Improve performance on `list_assets` - uses `HashMaps` instead of `Vec`

### Fixes

- Fixed apk won't extract when updating to newer version
- Added debug message on `find_game_config`

## 2.8.4

### Features

- Improve performance on catalog parsing side
- Improve color support checking

### API Changes

- From error will use transparent instead of `"{0}"`
- Added `Proxy` error

### Fixes

- Properly add proxy on version checking
- Cached contains ignore case filter method

## 2.8.2

### Fixes

- Fixed `needs_catalog_update` logic on cli
- Fixed color detection logic on config
- Fixed error messages duplications

## 2.8.0

### Features

- Improve performance on catalog parsing side

### API Changes

- Added `list_assets` helper to easily print or get the asset names
- Refactored codebase and remove redundant code

### Fixes

- Fix colors on certain terminals
  - Added a color detecting logic on `baad_utils` to prevent weird characters on `ansi` terminals (e.g command prompt)

## 2.7.5

### API Changes

- Fix `baad_utils::utils::file` exports redirecting to `baad::utils::json`
- Export `debug, error, file, info, warn, trace`


### Fixes

- It will download the rest of the files
    - Before it will just downloads the first found file not it will downloads the rest in JP


## 2.7.2

### Features

- Added direct file download
    - Allows for downloading `.bundle` files directly without downloading the Packed zip files in JP
    - This only works if you use `--filter` if not it will automatically download Packed zip instead
- Added proxy support
- Performance improvement
- Improve logging and errors

### API Changes

- Removed logging features
- Remove auto init of logging, now you need to `init_logging` or use `tracing_subscriber` to get logging
- `eyre` is not required for error handling
- Added `baad::helpers::error` to use for error handling
- Added `with_proxy` on `ApkFetcher`
- `ResourceDownloader` will now accepts `proxy`
- Replace Global version checking from `Google Play` to `Apkpure` instead
- Removed some redundant code

### Fixes

- Fixed android build can't even run
- Help just logs now it will use `clap` help instead
- Removed `uniffi` dependency when using as a library and compiling as bin

## v2.4.1

### Fixes

- Changed the cache path from `baad_utils` to `baad`

## v2.4.0

### Features

- Updated logging
    - Now uses `tracing` under the hood

### API Changes

- Now uses `eyre` for error handling
- Added logging configuration

### Fixes

- Remove the unnecessary panics

## v2.3.1

### Features

- Added `--ios` flag
    - Downloads assets from iOS build instead of default Android build
- Added `--teen` flag
    - Downloads teen-rated assets (Global only)
- Added way to do search for actual AssetBundle name in JP

### API Changes

- `ServerConfig::new()` now accepts optional `Platform` and `BuildType` parameters
- Added `ServerConfig::get_market_config()` method for Global server market details
- Updated path-related functions to use `&Path` instead of `&PathBuf` for better performance
- Added platform and build type configuration system
    - New `Platform` enum (Android, iOS)
    - New `BuildType` enum (Standard, Teen)

### Fixes

- Improve path loading performance

## 2.2.0

### Changes

- Due to Blue Archive Japan changed how AssetBundle downloads,
  it now downloads via Patch Packs aka zip files
- Added the ability to download and extract Global apk
- Updated the File Manager
    - You don't need to pass `FileManager::new()` anymore

### Fixes

- Updated extraction method
- Fixed error `Failed to decode response`
- Fixed `il2cpp` path when extracting

### Misc

- Bump BA-CY to `1.3.5`
- Remove redundant code

## v2.0.3

### Fixes

- Fix where the apk doesn't download if it's outdated
- Fix logs features not properly handled
    - Removing logs in now opt in

### Misc

- Exposed paris module
- Updated build ci

## v2.0.0

### Features

- Rewrite the entire codebase
    - This is a port of ba-ad and also a rewrite of ba-ad-rs
- Added library support
    - You can now use baad in any language you want or even use it in your rust projects
- Added **Global** support
- Added `--clean` to quickly clean the cache
- Added `--filter` to filter out specific files
    - Alternative to search mode
- Added `--filter-method`
    - You can now filter using `glob`, `fuzzy`, or `exact`
    - This is powered by `lazy-regex` and `nucleo` for performance
- Fully integrated with [`BA-CY`](https://github.com/Deathemonic/BA-CY)
- Replaced the old download manager with `trauma`
- Improved performance
- Improved logging

### Fixes

- Fixed APK will extract regardless you already did it
- Fixed catalog always fetches even though it's already been cached
- Fixed table bundles files are set to numbers instead of their actual name

### Misc

- Removed extract mode
- Removed search mode
- Removed custom catalog url
- Removed custom apk version
- Moved crypto to `BA-CY`
