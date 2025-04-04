<div align="center">
  <img src="baad/public/archive.png" width="500px" alt="logo">
  <h1>Blue Archive - Asset Downloader</h1>
</div>

A tool that downloads and extracts the **Blue Archive JP** `AssetBundles`, `TableBundles`, `MediaResources`.

It downloads them directly from the **Yostar Servers**.

## Requirements
- Python 3.10+
- .Net Runtime 6.0 (If you are using GNU/Linux or MacOS)

## Installation
Before doing anything, make sure you have [`git`](https://git-scm.com/downloads) installed.


- Windows (You might to close and reopen powershell after installing `uv`):

```sh
# Install UV
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"

# Install Blue Archive Asset Downloader
uv tool install git+https://github.com/Deathemonic/BA-AD
```

- Linux or MacOS:
```sh
# Install UV
curl -LsSf https://astral.sh/uv/install.sh | sh

# Install Blue Archive Asset Downloader
uv tool install git+https://github.com/Deathemonic/BA-AD
```

<details>
  <summary>Install using pip (Click to expand)</summary>
<br>

```sh
pip install git+https://github.com/Deathemonic/BA-AD
```
</details>

<br>

<details>
	<summary>Tutorial with Screenshots (Click to expand)</summary>
<br>

Before doing anything, make sure you have [`uv`](https://github.com/astral-sh/uv?tab=readme-ov-file#installation) installed.

1. Download the repository files

![stepone](baad/public/tutorial/step1.png)

2. Extract the downloaded zip and open the folder from the extracted zip

![steptwo](baad/public/tutorial/step2.png)

3. `Shift` + `Right Click` then click `Open PowerShell window here`

_alternatively you can open cmd then change directory to the folder_

![stepthree](baad/public/tutorial/step3.png)

<br>


4. Install the tool using `uv tool install .`

![steptwentytwo](baad/public/tutorial/step22.png)

5. You can now run it using the `uvx --from ba-ad baad` to see the usage. Check out [Usage](#usage) on how to use the tool.

![steptwentythree](baad/public/tutorial/step23.png)

<br>

4. Or you can install tool by typing `pip install .`

![stepfour](baad/public/tutorial/step4.png)


5. Your done just type `baad --help` to see the usage. Check out [Usage](#usage) on how to use the tool.

<br>

<details>
  <summary>Example Usage (Click to expand)</summary>
<br>

- Downloads the assetbundles
![stepfive](baad/public/tutorial/step5.png)
![stepsix](baad/public/tutorial/step6.png)

- Downloads the tablebundles
![stepseven](baad/public/tutorial/step7.png)

- Downloads the media reasources
![stepeight](baad/public/tutorial/step8.png)

- Output results
![stepnine](baad/public/tutorial/step9.png)
![stepthirteen](baad/public/tutorial/step13.png)

- Extracting assetbundles
![stepten](baad/public/tutorial/step10.png)
![stepeleven](baad/public/tutorial/step11.png)

- Extracting tablebundles
![steptwelve](baad/public/tutorial/step12.png)

</details>


</details>

## Usage

> [!IMPORTANT]
> Before you go downloading the game assets please read [Nexon's Terms and Service](https://m.nexon.com/terms/304) first. You are not allowed to sell the game files, models or use them for commercial purposes.

```
> baad --help
usage: baad.py [-h] [-v] [-u] [-g] {search,download,extract} ...

Blue Archive Asset Downloader

positional arguments:
  {search,download,extract}
    search              search mode
    download            download game files
    extract             extract game files

options:
  -h, --help            show this help message and exit
  -v, --version         show program's version number and exit
  -u, --update          force update the apk
  -g, --generate        generate the flatbuf schemas
```

#### Downloading

Two ways to download the `AssetBundles`, `TableBundles`, and `MediaResources`, you can use the `search` mode to search the files and download them one by one or you can use the `download` mode to download them all at once.


##### Search Mode
By passing `search` to the command it will initialize the search mode. It will display a telescope like interface to search.

- Press `ESC / Ctrl+C` to exit the search mode. 
- Press `Up / Down Arrow` to navigate the results.
- Press `Enter` to download.


```
> baad search --help
usage: baad search [-h] [--output OUTPUT]

options:
  -h, --help         show this help message and exit
  --output OUTPUT    output directory for the downloaded files (default: ./output)
```

- Preview

```
> baad search
╭────────────────────────────────────────────────────── [22065 matches] [Press ESC to exit] ───────────────────────────────────────────────────────╮
│ >                                                                                                                                                │
╰──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
╭──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╮
│  Category          Name                                                                                                                    Size  │
│  AssetBundles      academy-_mxload-2022-05-12_prefab_assets_all_4170120622.bundle                                                         1.1MB  │
│  AssetBundles      academy-_mxload-2022-09-14_prefab_assets_all_1177703018.bundle                                                       167.3KB  │
│  AssetBundles      academy-_mxload-2023-01-03_prefab_assets_all_79312310.bundle                                                         176.4KB  │
│  AssetBundles      academy-_mxload-2023-02-27_prefab_assets_all_1639057228.bundle                                                       172.7KB  │
│  AssetBundles      academy-_mxload-2023-03-14_prefab_assets_all_2700611036.bundle                                                       175.0KB  │
│  AssetBundles      academy-_mxload-2023-09-15_prefab_assets_all_3574084195.bundle                                                       178.5KB  │
│  AssetBundles      academy-_mxload-2024-08-02_prefab_assets_all_1586451891.bundle                                                       166.2KB  │
│  AssetBundles      akari_original_skill_frag-_mxload-2024-05-30_assets_all_235178830.bundle                                               7.7KB  │
│  AssetBundles      arms-_mxload-2022-05-12_prefab_assets_all_2242344387.bundle                                                           96.9KB  │
│  AssetBundles      arms-_mxload-2022-06-13_prefab_assets_all_169265965.bundle                                                            49.3KB  │
│  AssetBundles      arms-_mxload-2022-08-05_prefab_assets_all_721668755.bundle                                                            54.1KB  │
│  AssetBundles      arms-_mxload-2023-02-27_prefab_assets_all_1634731385.bundle                                                           46.2KB  │
│  AssetBundles      arms-_mxload-2024-02-27_prefab_assets_all_4171726177.bundle                                                           24.8KB  │
│  AssetBundles      ar_common-_mxload-2024-05-30_assets_all_3186116768.bundle                                                              2.6KB  │
│  AssetBundles      assets-spine-runtime-spine-unity-_mxdependency-2023-09-15_assets_all_1461670764.bundle                                21.6KB  │
│  AssetBundles      assets-_mx-3denvironments-_mxdependency-2022-05-12_assets_all_254839603.bundle                                       249.0KB  │
│  AssetBundles      assets-_mx-3denvironments-_mxdependency-2023-08-23_assets_all_2855529825.bundle                                        4.0KB  │
│  AssetBundles      assets-_mx-3denvironments-_mxdependency-2023-08-29_assets_all_4138896791.bundle                                      220.8KB  │
│  AssetBundles      assets-_mx-3dobject-airi_original_icecream-_mxdependency-2022-05-12_assets_all_2565193982.bundle                      45.8KB  │
│  AssetBundles      assets-_mx-3dobject-airi_original_icecream-_mxdependency-2022-05-12_mat_assets_all_2123000585.bundle                   3.0KB  │
│  AssetBundles      assets-_mx-3dobject-airi_original_icecream-_mxdependency-2022-05-12_psd_assets_all_3215411693.bundle                  16.0KB  │
│  AssetBundles      assets-_mx-3dobject-ayane_original_ex-_mxdependency-2022-05-12_assets_all_3241542591.bundle                            9.1KB  │
│  AssetBundles      assets-_mx-3dobject-ayane_original_ex-_mxdependency-2022-05-12_prefab_assets_all_4076567522.bundle                     5.3KB  │
│  AssetBundles      assets-_mx-3dobject-common_skill_healbomb_weapon-_mxdependency-2022-05-12_assets_all_1024083411.bundle                71.0KB  │
│  AssetBundles      assets-_mx-3dobject-common_skill_healbomb_weapon-_mxdependency-2022-05-12_mat_assets_all_1458339694.bundle             3.0KB  │
│  AssetBundles      assets-_mx-3dobject-common_skill_healbomb_weapon-_mxdependency-2022-05-12_psd_assets_all_2045859013.bundle             2.1KB  │
│  AssetBundles      assets-_mx-3dobject-common_skill_landmine-_mxdependency-2022-05-12_assets_all_450941517.bundle                        58.2KB  │
│                                                                                                                                                  │
│                                                                                                                                                  │
╰──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

##### Download Mode
By passing `download` to the command it will initialize the download mode. This is quite flexible than search mode because it alows you to control the _limitation of concurrent download_, _catalog url_, and _filter_.


```
> baad download --help
usage: baad download [-h] [--output OUTPUT] [--limit LIMIT] [--catalog CATALOG] [--filter FILTER] [--assets] [--tables] [--media] [-a]

options:
  -h, --help         show this help message and exit
  --output OUTPUT    output directory for the downloaded files (default: ./output)
  --limit LIMIT      set a limit the download limit (default: 5)
  --catalog CATALOG  force change the catalog url (will skip apk download)
  --filter FILTER    filter by name
  --assets           download the assetbundles
  --tables           download the tablebundles
  --media            download the mediaresources
  -a, --all          download all game files
```

You can pass `--update` to force update the game version.

```
> baad --update
```

Examples:

- Saves the files using Posix Path style
```
> baad download --tables --output ./Downloads
```

- Limit the concurrent download to 10 files
```
> baad download --assets --limit 10
```

- Downloads both `AssetBundles` and `MediaResources`
```
> baad download --assets --media
```

- Saves the files using Windows Path style
```
> baad download --media --output C:\Users\User\Documents
```

- Change the catalog url to a older catalog url
```
> baad download --assets --catalog https://prod-clientpatch.bluearchiveyostar.com/r67_jjjg51ngucokd90cuk4l_3
```

- Just download files that contains "Atsuko" in the name
```
> baad download --assets --filter "Atsuko"
```

> [!NOTE]
> If you have low connection and download say error, rerun the program again it will retry downloading the remaining files. Don't worry it will not download the already downloaded files again

#### Extracting

To extract the `AssetBundles`, `TableBundles`, and `MediaResources` we need to initialize extract mode, to do that we pass `extract` so it will be like this `baad extract`.


```
> baad extract --help

usage: baad extract [-h] [--path PATH] [--studio] [--assets] [--tables] [--media] [-a]

options:
  -h, --help   show this help message and exit
  --path PATH  path of the files that will be extracted
  --studio     uses the assetstudiomod as a backend for extracting the assetbundles
  --assets     extract the assetbundles
  --tables     extract the tablebundles
  --media      extract the mediaresources
  -a, --all    extract all game files
```
<br>

<details open>
	<summary>AssetBundles</summary>
<br>

  To extract assetbundles you need to pass `--assets` to extract the asset then pass `--path` to specify the path to extract the assetbundles. This will extract the assets files to the parent path folder then `AssetExtracted`.
  
  By default the extracter uses [UnityPy](https://github.com/ic005k/UnityPy) as it's backend but you can pass `--studio` to toggle [AssetStudioMod](https://github.com/aelurum/AssetStudio) as the backend.
  Due to the limitations for [UnityPy](https://github.com/ic005k/UnityPy) there's no way to extract the fbx, you could use [AssetStudioMod](https://github.com/aelurum/AssetStudio) backend to extract the fbx but any fbx associated with animator or animationclips will not be extracted.

  Generally you just pick the which assetbundle you need and put them in a seperate folder because it will extract all the assetbundles in the specified path. Extracting is RAM heavy.

  Example:
  - Use the selected assetbundles then extracts the assets to `./output/AssetExtracted`
  ```
  > baad extract --assets --path ./output/assetbundles
  ```
  
  - Uses AssetStudioMod as backend
  ```
  > baad extract --assets --path ./output/assetbundles --studio
  ```
</details>

> [!NOTE]
> This is not meant to be used for primary extracting the assetbundles, this is meant for quick and lazy extraction of the assetbundles and people uses GNU/Linux or MacOS.
> For proper extraction of the assetbundles you need to use [AssetStudioMod GUI](https://github.com/aelurum/AssetStudio) (Only runs in windows or use wine in linux [#696](https://github.com/Perfare/AssetStudio/issues/696)).

<br>

<details open>
	<summary>TableBundles</summary>
<br>

  Before extracting the tablebundles we need to generate the **flatbuffers** first.

  ```
  > baad --generate
  ```

  To extract tablebundles you need to pass `--tables` to extract the asset then pass `--path` to specify the path to extract the tablebundles. This will extract the assets files to the parent path folder then `TableExtracted`.


  Example:
  - Extracts the tables to `./output/TableExtracted`
  ```
  > baad extract --tables --path ./output/tablebundles
  ```
</details>

> [!NOTE]
> Note you may see warnings and some files didn't get parse, it's a known issue for now but most files will be parse just fine.

<details open>
	<summary>MediaResources</summary>
<br>
  Due to the recent Blue Archive update, which added korean dubs, the voices are now in a zip file.

  To extract mediaresources you need to pass `--media` to extract the asset then pass `--path` to specify the voices zip path. This will extract the audio files to the parent path folder then `MediaExtracted`.

  Example:
  - Extracts the voices to `./output/MediaExtracted`
  ```
  > baad extract --media --path ./output/MediaResources/GameData/Audio/VOC_JP
  ```
</details>

### What to do now?

You may be asking, I've downloaded everything so what do I do now?

Well anything you want, you can use assetbundles to extract the chibi models and port them into a different game like **Gmod** or **Left 4 Dead**. You can use the extracted tablebundles to view the character dialogs. You can use the media resources to listen to the character voice lines or the OST.

<details>
  <summary>Examples (Click to expand)</summary>
<br>

1. You can export the assetbundles using [AssetStudioMod](https://github.com/aelurum/AssetStudio)
![stepfourteen](baad/public/tutorial/step14.png)

- Then you can view the extracted model using [Blender](https://www.blender.org)
![stepfifteen](baad/public/tutorial/step15.png)

- Or you can convert the model to pmx to use with [MikuMikuDance](https://learnmmd.com)
![stepsixteen](baad/public/tutorial/step16.png)

- Or you can port the model to [Source](https://en.wikipedia.org/wiki/Source_(game_engine)) to use with **Gmod** or **Source Film Maker**
![stepseventeen](baad/public/tutorial/step24.png)


2. You can use the extracted tablebundles to view the game database

- You can view the character dialog
![stepeighteen](baad/public/tutorial/step18.png)

- You can view the charater skill info
![stepnineteen](baad/public/tutorial/step19.png)


3. You can use the download media resources to view the game backgrounds, cg, video, or music

- You can view the game backgrounds and CG
![steptwenty](baad/public/tutorial/step20.png)

- You can listen character voice lines and ost
![steptwentyone](baad/public/tutorial/step21.png)

</details>

### Global
If you want to downloaad the Global `AssetBundles`, `TableBundles`, and `MediaResources` then check out [BA-AD Rust](https://github.com/Deathemonic/BA-AD/tree/rust). It is a BA-AD written rust with global support.

To Install it just download one of the binaries at [Releases](https://github.com/Deathemonic/BA-AD/releases)

> [!NOTE]
> BA-AD Rust is still a work in progress, there are some stuff that is missing like the `search` and `extract` feature, it may also contains some bugs.

### Dump
To get `dump.cs` you need to manually decompile the `libil2cpp.so`. I recommend following the instructions from [Auto-Il2cppDumper](https://github.com/AndnixSH/Auto-Il2cppDumper) or [Zygisk-Il2CppDumper](https://github.com/Perfare/Zygisk-Il2CppDumper). Also I recommend using a emulator like **MuMuPlayer** to get easy root access.

But you your lazy you can use [Il2CppDumper](https://github.com/Perfare/Il2CppDumper) to dump it manually or automatically using a script. You gonna need the **Blue Archive JP** apk from [**ApkPure**](https://apkpure.com/blue-archive/com.YostarJP.BlueArchive), then rename .xapk to .zip and extract it. You will have multiple apks the apks you need is `config.arm64_v8a.apk` and `UnityDataAssetPack.apk` then rename .apk to .zip and extract it.

`libil2cpp.so` is located at extracted the `config.arm64_v8a.apk` then at `lib/arm64-v8a` and the `global-metadata.dat` is located at the extracted `UnityDataAssetPack.apk` then at `assets/bin/Data/Managed/Metadata`.

> [!NOTE]
> You will face a [ERROR: This file may be protected](https://github.com/Perfare/Il2CppDumper?tab=readme-ov-file#error-this-file-may-be-protected) by doing this method.

### Contributing
Don't like my [shitty code](https://shitcode.net/latest/language/python) and what to change it? Feel free to contribute by submitting a pull request or issue. Always appreciate the help.

### Acknowledgement
 - [hdk5/MoeXCOM](https://github.com/hdk5/MoeXCOM)
 - [respectZ/blue-archive-viewer](https://github.com/respectZ/blue-archive-viewer)
 - [fiseleo/Blue-Archive-JP-Downloader](https://github.com/fiseleo/Blue-Archive-JP-Downloader)
 - [K0lb3/Blue-Archive---Asset-Downloader](https://github.com/K0lb3/Blue-Archive---Asset-Downloader)
 - [lwd-temp/blue-archive-spine-production](https://github.com/lwd-temp/blue-archive-spine-production)
 - [aelurum/AssetStudio](https://github.com/aelurum/AssetStudio)


### Copyright
**Blue Archive** is a registered trademark of NAT GAMES Co., Ltd., NEXON Korea Corp., and Yostar, Inc. This project is not affiliated with, endorsed by, or connected to NAT GAMES Co., Ltd., NEXON Korea Corp., NEXON GAMES Co., Ltd., Yostar, Inc., or any of their subsidiaries or affiliates. All game assets, content, and materials are copyrighted by their respective owners and are used for informational and educational purposes only.
