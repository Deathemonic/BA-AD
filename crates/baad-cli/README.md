# baad-cli

Command-line interface for **Blue Archive - Asset Downloader**.

## Install

### Release

You can download the latest pre-build binaries at [Releases](https://github.com/Deathemonic/BA-AD/releases)

[Windows](https://github.com/Deathemonic/BA-AD/releases/latest/download/baad-windows-x86_64.zip) | [Linux](https://github.com/Deathemonic/BA-AD/releases/latest/download/baad-linux-x86_64.zip) | [MacOS](https://github.com/Deathemonic/BA-AD/releases/latest/download/baad-macos-aarch64.zip)

### Cargo

```shell
cargo install --git "https://github.com/Deathemonic/BA-AD" --locked baad-cli
```

## Usage

```shell
# Force update the APK and fetch the latest catalogs
baad --update

# Show detailed logs while updating local data
baad --update --verbose full

# Download JP table bundles into ./Downloads
baad download japan --tables --output ./Downloads

# Download Global media resources matching CH0230
baad download global --media --filter "ch0230"

# Clean cache data; useful when fixing stale or broken local state
baad --clean

# Download both asset bundles and media resources from JP
baad download japan --assets --media

# Download every JP resource category into a custom folder
baad download japan --assets --tables --media --output ./jp-all

# Download only Global table bundles
baad download global --tables --output ./global-tables

# Download China media resources into a region-specific folder
baad download china --media --output ./china-media

# Limit concurrent downloads
baad download global --assets --limit 15

# Increase retries for unstable connections
baad download japan --assets --retries 20

# Download through a local HTTP proxy
baad download global --media --proxy http://127.0.0.1:7890

# Download through a SOCKS proxy
baad download japan --assets --proxy socks5://127.0.0.1:1080

# Exact match one resource name
baad download japan --assets --filter "CH0230_home" --filter-method exact

# Match resources that start with a prefix
baad download global --media --filter "CH0230" --filter-method starts-with

# Match resources that end with a suffix
baad download china --media --filter ".mp4" --filter-method ends-with

# Use glob-style matching
baad download japan --assets --filter "*CH0230*" --filter-method glob

# Case-insensitive contains filter on CN assets
baad download china --assets --filter "CH0230" --filter-method contains-ignore-case

# Regex filter for multiple character resources
baad download china --media --filter "(ch0230|ch0255|hoshino).*battle" --filter-method regex

# Fuzzy search across all resource categories on JP
baad download japan --filter "ch0069" --filter-method fuzzy

# Download Global teen assets using iOS platform
baad download global --assets --platform ios --teen

# Download JP Windows asset bundles
baad download japan --assets --platform windows
```

## Command Overview

### `baad --help`

| Command/Option      | Short | Description                                               |
|---------------------|-------|-----------------------------------------------------------|
| `download`          |       | Download game files                                       |
| `help`              |       | Print this message or the help of the given subcommand(s) |
| `--update`          | `-u`  | Force update                                              |
| `--clean`           | `-c`  | Clean the cache                                           |
| `--verbose <LEVEL>` | `-v`  | Enable verbose output (`minimal`, `full`)                 |
| `--help`            | `-h`  | Print help                                                |
| `--version`         | `-V`  | Print version                                             |

### `baad download --help`

| Command  | Description                                               |
|----------|-----------------------------------------------------------|
| `global` | Download from Global server                               |
| `japan`  | Download from Japan server                                |
| `china`  | Download from China server                                |
| `help`   | Print this message or the help of the given subcommand(s) |

### `baad download {japan|global|china} --help`

| Option                            | Description                           | Default    | Possible Values                                                                                   |
|-----------------------------------|---------------------------------------|------------|---------------------------------------------------------------------------------------------------|
| `--assets`                        | Download asset bundles                |            |                                                                                                   |
| `--tables`                        | Download table bundles                |            |                                                                                                   |
| `--media`                         | Download media resources              |            |                                                                                                   |
| `--output <OUTPUT>`               | Output directory for downloaded files | `./output` |                                                                                                   |
| `--limit <LIMIT>`                 | Limit concurrent downloads            | `10`       |                                                                                                   |
| `--retries <RETRIES>`             | Retry attempts for failed downloads   | `10`       |                                                                                                   |
| `--filter <FILTER>`               | Filter by resource name               |            |                                                                                                   |
| `--filter-method <FILTER_METHOD>` | Filter method                         | `contains` | `exact`, `contains`, `regex`, `fuzzy`, `glob`, `contains-ignore-case`, `starts-with`, `ends-with` |
| `--proxy <PROXY>`                 | Proxy URL for downloads               |            |                                                                                                   |
| `--platform <PLATFORM>`           | Platform to download                  | `android`  | `android`, `ios`, `windows`                                                                       |
| `--teen`                          | Download Teen assets (Global only)    |            |                                                                                                   |
| `--help`                          | Print help                            |            |                                                                                                   |

## Building

```shell
git clone https://github.com/Deathemonic/BA-AD
cd BA-AD
cargo build -p baad-cli
```

---

<sub>**Copyright** - Blue Archive is a registered trademark of NAT GAMES Co., Ltd., NEXON Korea Corp., and Yostar, Inc.
This project is not affiliated with, endorsed by, or connected to NAT GAMES Co., Ltd., NEXON Korea Corp., NEXON GAMES
Co., Ltd., IODivision, Yostar, Inc., or any of their subsidiaries or affiliates. All game assets, content, and materials
are copyrighted by their respective owners and are used for informational and educational purposes only.</sub>
