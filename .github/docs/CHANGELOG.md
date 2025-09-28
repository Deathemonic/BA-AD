# Changelogs

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
  - Added a color detecting logic on `baad_core` to prevent weird characters on `ansi` terminals (e.g command prompt)

## 2.7.5

### API Changes

- Fix `baad_core::utils::file` exports redirecting to `baad::utils::json`
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

- Changed the cache path from `baad_core` to `baad`

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