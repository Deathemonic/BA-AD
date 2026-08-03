use lazy_regex::{Lazy, Regex, lazy_regex};
pub static REGEX_VERSION: Lazy<Regex> = lazy_regex!(r"\d+\.\d+\.\d+");

pub const GLOBAL_API_URL: &str = "https://api-pub.nexon.com/patch/v1.1/version-check";

pub const GLOBAL_PLAYSTORE_URL: &str =
    "https://apptopia.com/google-play/app/com.nexon.bluearchive/about";
pub const GLOBAL_APPSTORE_URL: &str = "https://apps.apple.com/us/app/blue-archive/id1571873795";

pub const GLOBAL_ANDROID_STANDARD_ID: &str = "com.nexon.bluearchive";
pub const GLOBAL_ANDROID_TEEN_ID: &str = "com.nexon.bluearchiveteen";
pub const GLOBAL_IOS_STANDARD_ID: &str = "1571873795";
pub const GLOBAL_IOS_TEEN_ID: &str = "6443698027";

pub const PLAYSTORE_CODE: &str = "playstore";
pub const APPSTORE_CODE: &str = "appstore";

pub const YOSTAR_BASE_URL: &str = "https://api-launcher-jp.yo-star.com";
pub const YOSTAR_GAME_BASE_CONFIG_PATH: &str = "/api/launcher/game/config";
pub const YOSTAR_GAME_JSON_CONFIG_PATH: &str = "/api/launcher/game/config/json";
pub const YOSTAR_DOMAIN_PATH: &str = "/api/launcher/advanced/game/download/cdn";
pub const YOSTAR_GAME_TAG: &str = "BlueArchive_JP";
pub const YOSTAR_SIGNATURE_DATA: &str = "DE7108E9B2842FD460F4777702727869";
pub const YOSTAR_VERSION: &str = "1.7.2";

pub const ROSTAR_VERSION_URL: &str = "https://bluearchive-cn.com/api/meta/setup";
pub const ROSTAR_STATE_URL: &str = "https://gs-api.bluearchive-cn.com/api/state";
pub const ROSTAR_PLATFORM_ID: &str = "1";
pub const ROSTAR_CHANNEL_ID: &str = "2";

pub const PLATFORM_NAME_ANDROID: &str = "Android";
pub const PLATFORM_NAME_IOS: &str = "iOS";
pub const PLATFORM_NAME_WINDOWS: &str = "Windows";

pub const API_FILENAME: &str = "api_data.json";
pub const GAME_CONFIG_PATTERN: &[u8] = &[
    0x47, 0x61, 0x6D, 0x65, 0x4D, 0x61, 0x69, 0x6E, 0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x00, 0x00,
    0x92, 0x03, 0x00, 0x00
];

pub const GLOBAL_APK_PATH: &str = "apk/BlueArchiveGlobal.xapk";
pub const JAPAN_APK_PATH: &str = "apk/BlueArchiveJP.xapk";

pub const CONFIG_APK: &str = "config.arm64_v8a.apk";
pub const LIBIL2CPP_PATH: &[&str] = &["lib", "arm64-v8a"];
pub const LIBIL2CPP_PATTERN: &str = "libil2cpp.so";
pub const ASSET_APK: &str = "UnityDataAssetPack.apk";
pub const JP_DATA_APK: &str = "com.YostarJP.BlueArchive.apk";
pub const GLOBAL_DATA_APK: &str = "com.nexon.bluearchive.apk";
pub const DATA_PATH: &[&str] = &["assets", "bin", "Data"];
pub const METADATA_PATH: &[&str] = &["assets", "bin", "Data", "Managed", "Metadata"];
pub const DATA_PATTERN: &str = "*";
pub const METADATA_PATTERN: &str = "global-metadata.dat";

pub const EXECUTABLE_NAME: &str = "baad";

pub const ASSET_BUNDLES: &str = "AssetBundles";
pub const TABLE_BUNDLES: &str = "TableBundles";
pub const MEDIA_RESOURCES: &str = "MediaResources";
pub const MEDIA_RESOURCES_WINDOWS: &str = "MediaResources-Windows";

pub const PATCH_PACK_ANDROID: &str = "Android_PatchPack";
pub const PATCH_PACK_IOS: &str = "iOS_PatchPack";
pub const PATCH_PACK_WINDOWS: &str = "Windows_PatchPack";

pub const CATALOG_PREFIX: &str = "Catalog/";
