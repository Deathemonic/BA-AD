use std::borrow::Cow;
use std::collections::HashMap;

use memorypack::MemoryPackable;
use serde::{Deserialize, Serialize};

fn default_platform() -> Cow<'static, str> { Cow::Borrowed("Android") }

fn default_build_type() -> Cow<'static, str> { Cow::Borrowed("Standard") }

fn default_group() -> Cow<'static, str> { Cow::Borrowed("GameData") }

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct JapanData {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub server_info_url: String,
    #[serde(default)]
    pub catalog_url: String,
    #[serde(default = "default_platform")]
    pub platform: Cow<'static, str>
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GlobalData {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub catalog_url: String,
    #[serde(default = "default_platform")]
    pub platform: Cow<'static, str>,
    #[serde(default = "default_build_type")]
    pub build_type: Cow<'static, str>
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ChinaData {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub catalog_url: String,
    #[serde(default)]
    pub resource_version: String,
    #[serde(default)]
    pub table_version: String,
    #[serde(default)]
    pub media_version: String,
    #[serde(default = "default_platform")]
    pub platform: Cow<'static, str>
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ApiData {
    #[serde(default)]
    pub japan: JapanData,
    #[serde(default)]
    pub global: GlobalData,
    #[serde(default)]
    pub china: ChinaData
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JapanAddressable {
    pub connection_groups: Vec<ConnectionGroup>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConnectionGroup {
    pub name: String,
    pub management_data_url: String,
    pub is_production_addressables: bool,
    pub api_url: String,
    pub gateway_url: String,
    pub kibana_log_url: String,
    pub prohibited_word_black_list_uri: String,
    pub prohibited_word_white_list_uri: String,
    pub customer_service_url: String,
    pub override_connection_groups: Vec<OverrideConnectionGroup>,
    pub bundle_version: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OverrideConnectionGroup {
    pub name: String,
    pub addressables_catalog_url_root: String
}

#[derive(Deserialize, Serialize)]
pub struct GlobalAddressable {
    pub api_version: String,
    pub market_game_id: String,
    pub latest_build_version: String,
    pub latest_build_number: String,
    pub min_build_version: String,
    pub min_build_number: String,
    pub patch: GlobalPatch
}

#[derive(Deserialize, Serialize)]
pub struct GlobalPatch {
    pub patch_version: i32,
    pub resource_path: String,
    pub bdiff_path: Vec<HashMap<String, String>>
}

#[derive(Serialize, Deserialize)]
pub struct GlobalCatalog {
    pub id: i32,
    pub market_game_id: String,
    pub build_id: Vec<i32>,
    pub patch_version: i32,
    pub name: String,
    pub patch_state: String,
    pub security_checked: bool,
    pub multi_language: bool,
    pub multi_texture_encode: bool,
    pub multi_texture_quality: bool,
    pub description: String,
    pub register: String,
    pub register_date: String,
    pub updater: String,
    pub update_date: String,
    pub compress: bool,
    pub size: i64,
    pub count: i32,
    pub use_multi_resource: bool,
    pub category: Category,
    pub category_mapping: Vec<CategoryMapping>,
    pub resources: Vec<Resource>
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub lang: Option<String>,
    pub texture_encode_type: Option<String>,
    pub texture_quality_level: Option<String>,
    pub group: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct CategoryMapping {
    pub group: String,
    pub paths: Vec<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Resource {
    #[serde(default = "default_group")]
    pub group: Cow<'static, str>,
    pub resource_path: String,
    pub resource_size: i64,
    pub resource_hash: String
}

#[derive(Deserialize)]
pub struct GameBaseConfig {
    pub game_lowest_version: String,
    pub game_latest_version: String,
    pub game_latest_file_path: String,
    pub game_start_exe_name: String,
    pub game_file_size: String,
    pub game_file_size_type: String,
    pub crc64: String,
    pub size: i64,
    pub file_url: String,
    pub decompression_size: String,
    pub config_id: i32,
    #[serde(default)]
    pub game_start_params: Vec<String>
}

#[derive(Deserialize)]
pub struct GameJsonConfig {
    pub url: String
}

#[derive(Deserialize)]
pub struct GameFile {
    pub path: String,
    pub hash: String,
    pub size: String
}

#[derive(Deserialize)]
pub struct GameJsonData {
    pub source: String,
    pub file: Vec<GameFile>
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChinaState {
    pub addressables_catalog_url_roots: Vec<String>,
    pub resource_version: String,
    pub table_version: String,
    pub media_version: String,
    pub patch_version: String,
    pub state: i32,
    pub is_open_pre_download: bool
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChinaTableManifest {
    pub table: HashMap<String, ChinaTableEntry>
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChinaTableEntry {
    pub name: String,
    pub size: i64,
    pub crc: String,
    #[serde(rename = "isInbuild")]
    pub is_inbuild: bool,
    #[serde(rename = "isChanged")]
    pub is_changed: bool,
    pub is_prologue: bool,
    pub is_split_download: bool,
    pub includes: Option<Vec<String>>
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChinaBundleManifest {
    pub bundle_files: Vec<ChinaBundleFile>
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChinaBundleFile {
    pub name: String,
    pub size: i64,
    pub is_prologue: bool,
    pub crc: String,
    pub is_split_download: bool
}

pub struct ChinaMediaEntry {
    pub path: String,
    pub hash: String,
    pub media_type: i32,
    pub bytes: i64
}

#[derive(Deserialize)]
pub struct Domain {
    pub primary_cdn: String,
    pub back_up_cdn: String
}

#[derive(Debug, Clone)]
pub enum HashValue {
    Crc(i64),
    Md5(String)
}

impl HashValue {
    pub fn as_string(&self) -> String {
        match self {
            HashValue::Crc(crc) => crc.to_string(),
            HashValue::Md5(md5) => md5.clone()
        }
    }
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Media {
    pub path: String,
    pub file_name: String,
    pub bytes: i64,
    pub crc: i64,
    pub is_prologue: bool,
    pub is_split_download: bool,
    pub media_type: i32
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TableBundle {
    pub name: String,
    pub size: i64,
    pub crc: i64,
    #[serde(rename = "isInbuild")]
    pub is_inbuild: bool,
    #[serde(rename = "isChanged")]
    pub is_changed: bool,
    pub is_prologue: bool,
    pub is_split_download: bool,
    pub includes: Vec<String>
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TablePatchPack {
    pub name: String,
    pub size: i64,
    pub crc: i64,
    pub is_prologue: bool,
    pub bundle_files: Vec<TableBundle>
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MediaCatalog {
    pub table: HashMap<String, Media>
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TableCatalog {
    pub table: HashMap<String, TableBundle>,
    pub table_pack: HashMap<String, TablePatchPack>
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BundlePatchPackInfo {
    pub milestone: String,
    pub patch_version: i32,
    pub full_patch_packs: Vec<BundlePatchPack>,
    pub update_packs: Vec<BundlePatchPack>
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BundlePatchPack {
    pub pack_name: String,
    pub pack_size: i64,
    pub crc: i64,
    pub is_prologue: bool,
    pub is_split_download: bool,
    pub bundle_files: Vec<BundleFile>
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BundleFile {
    pub name: String,
    pub size: i64,
    pub is_prologue: bool,
    pub crc: i64,
    pub is_split_download: bool,
    pub file_hash: u64,
    pub signature: String
}

#[derive(Debug, Clone)]
pub struct DownloadAsset {
    pub url: String,
    pub path: String,
    pub hash: HashValue,
    pub size: i64,
    pub bundle_files: Vec<String>
}

#[derive(Debug, Clone)]
pub struct DownloadMedia {
    pub url: String,
    pub path: String,
    pub hash: HashValue,
    pub size: i64
}

#[derive(Debug, Clone)]
pub struct DownloadTable {
    pub url: String,
    pub path: String,
    pub hash: HashValue,
    pub size: i64,
    pub bundle_files: Vec<String>
}

#[derive(Debug, Clone)]
pub struct Downloads {
    pub assets: Vec<DownloadAsset>,
    pub tables: Vec<DownloadTable>,
    pub media: Vec<DownloadMedia>
}
