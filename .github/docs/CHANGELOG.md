# Changelogs

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
- Fixed il2cpp path when extracting

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
- Fully integrated with `BA-CY`
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