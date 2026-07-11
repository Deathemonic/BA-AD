use strum::{AsRefStr, EnumString, IntoStaticStr};

use crate::consts::*;
use crate::error::ServerConfigError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, AsRefStr, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
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

    pub const fn media_path(self) -> &'static str {
        match self {
            Self::Android | Self::Ios => MEDIA_RESOURCES,
            Self::Windows => MEDIA_RESOURCES_WINDOWS
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Android => PLATFORM_NAME_ANDROID,
            Self::Ios => PLATFORM_NAME_IOS,
            Self::Windows => PLATFORM_NAME_WINDOWS
        }
    }

    pub fn ensure_supported(self) -> Result<(), ServerConfigError> {
        match self {
            Self::Windows => Err(ServerConfigError::WindowsNotSupported),
            _ => Ok(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, AsRefStr, IntoStaticStr)]
pub enum BuildType {
    Standard,
    Teen
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
            (Platform::Windows, _) => return Err(ServerConfigError::WindowsNotSupported)
        };

        Ok(Self {
            market_game_id,
            market_code
        })
    }
}
