use crate::helpers::error::ServerConfigError;

use lazy_regex::{lazy_regex, Lazy, Regex};
use reqwest::header::{HeaderMap, HeaderValue};
use std::borrow::Cow;
use std::rc::Rc;

pub static JAPAN_REGEX_URL: Lazy<Regex> = lazy_regex!(
    r"(X?APKJ)..(https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*))"
);
pub const GLOBAL_API_URL: &str = "https://api-pub.nexon.com/patch/v1.1/version-check";

pub static REGEX_VERSION: Lazy<Regex> =
    lazy_regex!(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)");

pub const GLOBAL_VERSION_URL: &str =
    "https://api.pureapk.com/m/v3/cms/app_version?hl=en-US&package_name=com.nexon.bluearchive";
pub const JAPAN_VERSION_URL: &str =
    "https://api.pureapk.com/m/v3/cms/app_version?hl=en-US&package_name=com.YostarJP.BlueArchive";

pub const GLOBAL_APK_PATH: &str = "apk/BlueArchiveGlobal.xapk";
pub const JAPAN_APK_PATH: &str = "apk/BlueArchiveJP.xapk";

pub const GLOBAL_ANDROID_STANDARD_ID: &str = "com.nexon.bluearchive";
pub const GLOBAL_ANDROID_TEEN_ID: &str = "com.nexon.bluearchiveteen";
pub const GLOBAL_IOS_STANDARD_ID: &str = "1571873795";
pub const GLOBAL_IOS_TEEN_ID: &str = "6443698027";
pub const PLAYSTORE_CODE: &str = "playstore";
pub const APPSTORE_CODE: &str = "appstore";

pub const API_FILENAME: &str = "api_data.json";
pub const GAME_CONFIG_PATTERN: &[u8] = &[
    0x47, 0x61, 0x6D, 0x65, 0x4D, 0x61, 0x69, 0x6E, 0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x00, 0x00,
    0x92, 0x03, 0x00, 0x00,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerRegion {
    Global,
    Japan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Android,
    Ios,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildType {
    Standard,
    Teen,
}

pub struct ServerConfig {
    pub region: ServerRegion,
    pub platform: Platform,
    pub build_type: BuildType,
    pub version_url: Cow<'static, str>,
    pub apk_path: Cow<'static, str>,
}

pub struct MarketConfig {
    pub market_game_id: Cow<'static, str>,
    pub market_code: Cow<'static, str>,
}

impl Platform {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Platform::Android => "Android",
            Platform::Ios => "Ios",
        }
    }
}

impl BuildType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            BuildType::Standard => "Standard",
            BuildType::Teen => "Teen",
        }
    }
}

impl ServerConfig {
    pub fn new(
        server: ServerRegion,
        platform: Option<Platform>,
        build_type: Option<BuildType>,
    ) -> Result<Rc<Self>, ServerConfigError> {
        let platform = platform.unwrap_or(Platform::Android);
        let build_type = build_type.unwrap_or(BuildType::Standard);

        if build_type == BuildType::Teen && server != ServerRegion::Global {
            return Err(ServerConfigError::TeenNotAvailable);
        }

        let config = match server {
            ServerRegion::Global => Self {
                region: server,
                platform,
                build_type,
                version_url: GLOBAL_VERSION_URL.into(),
                apk_path: GLOBAL_APK_PATH.into(),
            },
            ServerRegion::Japan => Self {
                region: server,
                platform,
                build_type,
                version_url: JAPAN_VERSION_URL.into(),
                apk_path: JAPAN_APK_PATH.into(),
            },
        };

        Ok(Rc::new(config))
    }

    pub fn get_market_config(&self) -> Option<MarketConfig> {
        match self.region {
            ServerRegion::Global => {
                let (market_game_id, market_code) = match (self.platform, self.build_type) {
                    (Platform::Android, BuildType::Standard) => {
                        (GLOBAL_ANDROID_STANDARD_ID, PLAYSTORE_CODE)
                    }
                    (Platform::Android, BuildType::Teen) => {
                        (GLOBAL_ANDROID_TEEN_ID, PLAYSTORE_CODE)
                    }
                    (Platform::Ios, BuildType::Standard) => (GLOBAL_IOS_STANDARD_ID, APPSTORE_CODE),
                    (Platform::Ios, BuildType::Teen) => (GLOBAL_IOS_TEEN_ID, APPSTORE_CODE),
                };

                Some(MarketConfig {
                    market_game_id: market_game_id.into(),
                    market_code: market_code.into(),
                })
            }
            ServerRegion::Japan => None,
        }
    }
}

pub fn apk_headers() -> HeaderMap {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("x-cv", HeaderValue::from_static("3172501"));
    headers.insert("x-sv", HeaderValue::from_static("29"));
    headers.insert(
        "x-abis",
        HeaderValue::from_static("arm64-v8a,armeabi-v7a,armeabi"),
    );
    headers.insert("x-gp", HeaderValue::from_static("1"));
    headers
}
