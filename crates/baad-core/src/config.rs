use lazy_regex::{Lazy, Regex, lazy_regex};

use crate::error::ServerConfigError;

pub static JAPAN_REGEX_URL: Lazy<Regex> = lazy_regex!(
    r"(X?APKJ)..(https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*))"
);
pub const GLOBAL_API_URL: &str = "https://api-pub.nexon.com/patch/v1.1/version-check";

pub static REGEX_VERSION: Lazy<Regex> =
    lazy_regex!(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)");

pub const GLOBAL_VERSION_URL: &str =
    "https://api.pureapk.com/m/v3/cms/app_version?hl=en-US&package_name=com.nexon.bluearchive";
pub const PLAYSTORE_VERSION_URL: &str =
    "https://play.google.com/store/apps/details?id=com.nexon.bluearchive";
pub static PLAYSTORE_REGEX_VERSION: Lazy<Regex> = lazy_regex!(r"\d\.\d{2}\.\d{6}");
pub const JAPAN_VERSION_URL: &str =
    "https://api.pureapk.com/m/v3/cms/app_version?hl=en-US&package_name=com.YostarJP.BlueArchive";

pub const GLOBAL_APK_PATH: &str = "apk/BlueArchiveGlobal.xapk";
pub const JAPAN_APK_PATH: &str = "apk/BlueArchiveJP.xapk";

pub const GLOBAL_ANDROID_STANDARD_ID: &str = "com.nexon.bluearchive";
pub const GLOBAL_ANDROID_TEEN_ID: &str = "com.nexon.bluearchiveteen";
pub const GLOBAL_IOS_STANDARD_ID: &str = "1571873795";
pub const GLOBAL_IOS_TEEN_ID: &str = "6443698027";

pub const YOSTAR_BASE_URL: &str = "https://api-launcher-jp.yo-star.com";
pub const YOSTAR_GAME_BASE_CONFIG_PATH: &str = "/api/launcher/game/config";
pub const YOSTAR_GAME_JSON_CONFIG_PATH: &str = "/api/launcher/game/config/json";
pub const YOSTAR_DOMAIN_PATH: &str = "/api/launcher/advanced/game/download/cdn";
pub const YOSTAR_GAME_TAG: &str = "BlueArchive_JP";
pub const YOSTAR_SIGNATURE_DATA: &str = "DE7108E9B2842FD460F4777702727869";
pub const YOSTAR_VERSION: &str = "1.7.2";
pub const PLAYSTORE_CODE: &str = "playstore";
pub const APPSTORE_CODE: &str = "appstore";

pub const API_FILENAME: &str = "api_data.json";
pub const GAME_CONFIG_PATTERN: &[u8] = &[
    0x47, 0x61, 0x6D, 0x65, 0x4D, 0x61, 0x69, 0x6E, 0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x00, 0x00,
    0x92, 0x03, 0x00, 0x00
];

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

pub const PATCH_PACK_ANDROID: &str = "Android_PatchPack";
pub const PATCH_PACK_IOS: &str = "iOS_PatchPack";
pub const PATCH_PACK_WINDOWS: &str = "Windows_PatchPack";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Android,
    Ios,
    Windows
}

impl Platform {
    pub const fn patch_pack(self) -> &'static str {
        match self {
            Self::Android => PATCH_PACK_ANDROID,
            Self::Ios => PATCH_PACK_IOS,
            Self::Windows => PATCH_PACK_WINDOWS
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Platform::Android => "Android",
            Platform::Ios => "Ios",
            Platform::Windows => "Windows"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildType {
    Standard,
    Teen
}

impl BuildType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            BuildType::Standard => "Standard",
            BuildType::Teen => "Teen"
        }
    }
}

pub struct MarketConfig {
    pub market_game_id: &'static str,
    pub market_code: &'static str
}

impl MarketConfig {
    pub fn for_global(
        platform: Platform,
        build_type: BuildType
    ) -> Result<Self, ServerConfigError> {
        let (market_game_id, market_code) = match (platform, build_type) {
            (Platform::Android, BuildType::Standard) => {
                (GLOBAL_ANDROID_STANDARD_ID, PLAYSTORE_CODE)
            }
            (Platform::Android, BuildType::Teen) => (GLOBAL_ANDROID_TEEN_ID, PLAYSTORE_CODE),
            (Platform::Ios, BuildType::Standard) => (GLOBAL_IOS_STANDARD_ID, APPSTORE_CODE),
            (Platform::Ios, BuildType::Teen) => (GLOBAL_IOS_TEEN_ID, APPSTORE_CODE),
            (Platform::Windows, _) => return Err(ServerConfigError::UnsupportedCombination)
        };

        Ok(Self {
            market_game_id,
            market_code
        })
    }
}
