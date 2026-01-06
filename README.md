<div>
  <img src=".github/resources/archive.png" width="4062" alt="logo">
</div>

# Blue Archive - Asset Downloader
A tool and library that downloads the latest **Blue Archive** assets.

> [!WARNING]  
> This is a unfinished version of `BA-AD` don't use unless your a developer/contributor.

## Install

### Release
You can download the latest pre-build binaries at [Releases](https://github.com/Deathemonic/BA-AD/releases)

[Windows](https://github.com/Deathemonic/BA-AD/releases/latest/download/baad-windows-x86_64.zip) | [Linux](https://github.com/Deathemonic/BA-AD/releases/latest/download/baad-linux-x86_64.zip) | [MacOS](https://github.com/Deathemonic/BA-AD/releases/latest/download/baad-macos-aarch64.zip) 

### Cargo
```shell
cargo install --git "https://github.com/Deathemonic/BA-AD" --locked --release
```

## Usage

Download all assets from `JP` server
```shell
baad download japan
```

Download all assets from `Global` server
```shell
baad download global
```

### Examples

```shell
# Force update the APK and fetches the latest catalogs
baad --update

# Cleans everything, can also use to fix any problems
baad --clean

# Downloads the TableBundles from JP server and save it in a folder named Downloads
baad download japan --tables --output ./Downloads

# Downloads the MediaResources from the Global server that contains CH0230 in it
baad download global --media --filter "ch0230"

# Downloads both AssetBundles and MediaResources from JP Server
baad download japan --assets --media

# Downloads the AssetBundles with a limit of 15 concurrent downloads
baad download global --assets --limit 15 

# Downloads the AssetBundles from JP server that contains CH0230 in it using contains but ignore case 
baad download japan --assets --filter "CH0230" --filter-method contains-ignore-case

# Downloads all AssetBundles, TableBundles, and MediaResources from JP server that contains ch0069 in it using fuzzy search  
baad download japan --filter "ch0069" --filter-method fuzzy

# Downloads all teen AssetBundles from the Global server using iOS platform
baad download global --assets --ios --teen

# Downloads all AssetBundles from the JP server using iOS platform 
baad download japan --assets --ios
```

<details>
  <summary>Command Line</summary>

### `baad --help`

| Command/Option | Short | Description                                               |
|----------------|-------|-----------------------------------------------------------|
| `download`     |       | Download game files                                       |
| `help`         |       | Print this message or the help of the given subcommand(s) |
| `--update`     | `-u`  | Force update                                              |
| `--clean`      | `-c`  | Cleans the cache                                          |
| `--verbose`    | `-v`  | Enable verbose output                                     |
| `--help`       | `-h`  | Print help                                                |
| `--version`    | `-V`  | Print version                                             |

---

### `baad download --help`

| Command  | Description                                               |
|----------|-----------------------------------------------------------|
| `global` | Download from Global server                               |
| `japan`  | Download from Japan server                                |
| `help`   | Print this message or the help of the given subcommand(s) |

---

### `baad download {japan|global} --help`

| Option                            | Description                                   | Default    | Possible Values                                                                                   |
|-----------------------------------|-----------------------------------------------|------------|---------------------------------------------------------------------------------------------------|
| `--assets`                        | Download the assetbundles                     |            |                                                                                                   |
| `--tables`                        | Download the tablebundles                     |            |                                                                                                   |
| `--media`                         | Download the mediaresources                   |            |                                                                                                   |
| `--output <OUTPUT>`               | Output directory for the downloaded files     | `./output` |                                                                                                   |
| `--limit <LIMIT>`                 | Set a limit on the concurrent downloads       | `10`       |                                                                                                   |
| `--retries <RETRIES>`             | Number of retry attempts for failed downloads | `10`       |                                                                                                   |
| `--filter <FILTER>`               | Filter by name                                |            |                                                                                                   |
| `--filter-method <FILTER_METHOD>` | Filter method to use                          | `contains` | `exact`, `contains`, `regex`, `fuzzy`, `glob`, `contains-ignore-case`, `starts-with`, `ends-with` |
| `--proxy <PROXY>`                 | Proxy URL for downloads                       |            |                                                                                                   |
| `--ios`                           | Use iOS build instead of Android              |            |                                                                                                   | 
| `--teen`                          | Download Teen assets (Global only)            |            |                                                                                                   | 
| `--help`                          | Print help                                    |            |                                                                                                   |


</details>

## Building

1. Install [rustup](https://rustup.rs)
2. Clone this repository
```sh
git clone https://github.com/Deathemonic/BA-AD
cd BA-AD
```
3. Build using `cargo`
```sh
cargo build
```

## Library
```toml
baad = { git = "https://github.com/Deathemonic/BA-AD" }
```

For more info check out [Library](.github/docs/LIBRARY.md)

### Other Projects

- [BA-AX](https://github.com/Deathemonic/BA-AX): A tool and library that extracts **Blue Archive** assets.
- [BA-MU](https://github.com/Deathemonic/BA-MU): A tool that re-dump AssetBundle for **Blue Archive**.
- [BA-FB](https://github.com/Deathemonic/BA-FB): A tool for dumping and generating **Blue Archive** flatbuffers.
- [BA-CY](https://github.com/Deathemonic/BA-CY): Library for handling **Blue Archive** catalogs, tables, serialization/deserialization, encryption, and hashing.


### Contributing
Don't like my [shitty code](https://www.reddit.com/r/programminghorror) and what to change it? Feel free to contribute by submitting a **pull request** or **issue**. Always appreciate the help.


### Acknowledgement
- [hdk5/MoeXCOM](https://github.com/hdk5/MoeXCOM)
- [respectZ/blue-archive-viewer](https://github.com/respectZ/blue-archive-viewer)
- [fiseleo/Blue-Archive-JP-Downloader](https://github.com/fiseleo/Blue-Archive-JP-Downloader)
- [K0lb3/Blue-Archive---Asset-Downloader](https://github.com/K0lb3/Blue-Archive---Asset-Downloader)
- [lwd-temp/blue-archive-spine-production](https://github.com/lwd-temp/blue-archive-spine-production)
- [aelurum/AssetStudio](https://github.com/aelurum/AssetStudio)

### Copyright
Blue Archive is a registered trademark of NAT GAMES Co., Ltd., NEXON Korea Corp., and Yostar, Inc.
This project is not affiliated with, endorsed by, or connected to NAT GAMES Co., Ltd., NEXON Korea Corp., NEXON GAMES Co., Ltd., IODivision, Yostar, Inc., or any of their subsidiaries or affiliates.
All game assets, content, and materials are copyrighted by their respective owners and are used for informational and educational purposes only.
