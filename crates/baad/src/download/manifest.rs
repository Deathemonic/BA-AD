//! Machine-readable download plans and their results.
//!
//! A [`Manifest`] lists exactly what a download would write: one entry per
//! output file, with the URL, expected size and checksum, and for files
//! inside a ZIP pack, the member to extract. It can be saved, inspected or
//! trimmed by other tools and handed back to download the same files without
//! fetching the catalog again. A [`Report`] records the outcome per file.

use std::collections::HashMap;
use std::fmt;
use std::path::Path;

use baad_dm::{Download, HashType, Summary};
use baad_shared::{DownloadStatus, Downloads};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::fs;

use crate::download::{ResourceFilter, converter};
use crate::error::CatalogError;

/// Version of the [`Manifest`] and [`Report`] JSON formats.
pub const MANIFEST_SCHEMA: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub schema: u32,
    /// `japan`, `global` or `china`.
    pub region: String,
    /// `android`, `ios` or `windows`.
    pub platform: String,
    /// `standard` or `teen`.
    pub build: String,
    /// Client version the catalog was resolved for, when known.
    pub version: Option<String>,
    /// Catalog root or base URL that resource URLs were built from.
    pub source: String,
    pub resources: Vec<ManifestEntry>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceKind {
    Assets,
    Tables,
    Media
}

impl ResourceKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Assets => "Assets",
            Self::Tables => "Tables",
            Self::Media => "Media"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ManifestHash {
    Md5,
    Crc32
}

impl From<ManifestHash> for HashType {
    fn from(value: ManifestHash) -> Self {
        match value {
            ManifestHash::Md5 => Self::Md5,
            ManifestHash::Crc32 => Self::Crc32
        }
    }
}

/// One output file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEntry {
    pub kind: ResourceKind,
    /// Output path relative to the output directory, separated by `/`.
    pub path: String,
    pub url: String,
    /// Expected size of the downloaded file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    /// Expected checksum of the downloaded file; requires `hash_type`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash_type: Option<ManifestHash>,
    /// Extract only this member of the ZIP at `url`. Members are checked
    /// against the archive's own CRC32 and size, so `size` and `hash` are
    /// not used for them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    /// Names of the bundles packed in this file, when it is a ZIP pack.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>
}

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Unsupported manifest schema {0}; expected {MANIFEST_SCHEMA}")]
    UnsupportedSchema(u32),

    #[error("Manifest is for {found}, not {expected}")]
    RegionMismatch { expected: String, found: String },

    #[error("Unsafe output path in manifest: {0:?}")]
    UnsafePath(String),

    #[error("Invalid URL for {path}: {url}")]
    InvalidUrl { path: String, url: String },

    #[error("Hash without hash_type for {0}")]
    MissingHashType(String),

    #[error("Invalid ZIP member for {0}")]
    InvalidMember(String),

    #[error("Conflicting entries for output path in manifest: {0}")]
    DuplicatePath(String)
}

impl Manifest {
    pub async fn load(path: &Path) -> Result<Self, CatalogError> {
        Ok(serde_json::from_slice(&fs::read(path).await?)?)
    }

    pub async fn save(&self, path: &Path) -> Result<(), CatalogError> {
        save_json(path, self).await
    }

    /// Checks the whole manifest before anything is downloaded.
    pub fn validate(&self, region: &str) -> Result<(), ManifestError> {
        if self.schema != MANIFEST_SCHEMA {
            return Err(ManifestError::UnsupportedSchema(self.schema));
        }
        if self.region != region {
            return Err(ManifestError::RegionMismatch {
                expected: region.into(),
                found: self.region.clone()
            });
        }
        let mut seen = HashMap::new();
        for entry in &self.resources {
            entry.to_download()?;
            if seen.insert(entry.path.as_str(), entry).is_some_and(|previous| previous != entry) {
                return Err(ManifestError::DuplicatePath(entry.path.clone()));
            }
        }
        Ok(())
    }
}

impl ManifestEntry {
    /// Builds the download, rejecting paths that could leave the output
    /// directory and hashes whose algorithm is not stated.
    pub fn to_download(&self) -> Result<Download, ManifestError> {
        if !is_safe_relative_path(&self.path) {
            return Err(ManifestError::UnsafePath(self.path.clone()));
        }
        let url = Url::parse(&self.url)
            .ok()
            .filter(|url| matches!(url.scheme(), "http" | "https"))
            .ok_or_else(|| ManifestError::InvalidUrl {
                path: self.path.clone(),
                url: self.url.clone()
            })?;
        if self.hash.is_some() && self.hash_type.is_none() {
            return Err(ManifestError::MissingHashType(self.path.clone()));
        }
        if self.member.as_deref().is_some_and(str::is_empty) {
            return Err(ManifestError::InvalidMember(self.path.clone()));
        }
        Ok(Download::builder()
            .url(url)
            .filename(self.path.clone())
            .maybe_hash(self.hash.clone())
            .maybe_hash_type(self.hash_type.map(Into::into))
            .maybe_size(self.size)
            .maybe_target_file(self.member.clone())
            .build())
    }

    fn from_download(kind: ResourceKind, download: &Download, members: &[String]) -> Self {
        let member = download.target_file.clone();
        let archive = member.is_none();
        Self {
            kind,
            path: download.filename.clone(),
            url: download.url.to_string(),
            size: download.size.filter(|_| archive),
            hash: download.hash.clone().filter(|_| archive),
            hash_type: download.hash_type.and_then(manifest_hash).filter(|_| archive),
            member,
            members: if archive { members.to_vec() } else { Vec::new() }
        }
    }
}

const fn manifest_hash(hash_type: HashType) -> Option<ManifestHash> {
    match hash_type {
        HashType::Md5 => Some(ManifestHash::Md5),
        HashType::Crc32 => Some(ManifestHash::Crc32),
        _ => None
    }
}

/// Writes through a temporary file so readers never see a partial document.
async fn save_json(path: &Path, value: &impl Serialize) -> Result<(), CatalogError> {
    let mut json = serde_json::to_vec_pretty(value)?;
    json.push(b'\n');
    if let Some(parent) = path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
        fs::create_dir_all(parent).await?;
    }
    let mut temporary = path.as_os_str().to_owned();
    temporary.push(".tmp");
    fs::write(&temporary, json).await?;
    fs::rename(&temporary, path).await?;
    Ok(())
}

/// Rejects empty, absolute and parent segments, and characters that change
/// meaning on Windows.
fn is_safe_relative_path(path: &str) -> bool {
    path.split('/')
        .all(|segment| !matches!(segment, "" | "." | "..") && !segment.contains(['\\', ':', '\0']))
}

/// Resolves `downloads` and `filter` into the files a download would write.
pub fn plan(downloads: &Downloads, filter: Option<&ResourceFilter>) -> Vec<ManifestEntry> {
    let mut members: HashMap<&str, &[String]> = HashMap::new();
    for asset in &downloads.assets {
        members.insert(&asset.path, &asset.bundle_files);
    }
    for table in &downloads.tables {
        members.insert(&table.path, &table.bundle_files);
    }
    let entries = |kind, converted: Vec<Download>| {
        converted
            .into_iter()
            .map(|download| {
                let packed = members.get(download.filename.as_str()).copied().unwrap_or_default();
                ManifestEntry::from_download(kind, &download, packed)
            })
            .collect::<Vec<_>>()
    };

    let mut resources =
        entries(ResourceKind::Assets, converter::convert_assets(&downloads.assets, filter));
    resources.extend(entries(
        ResourceKind::Tables,
        converter::convert_tables(&downloads.tables, filter)
    ));
    resources
        .extend(entries(ResourceKind::Media, converter::convert_media(&downloads.media, filter)));
    resources
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileStatus {
    Downloaded,
    Skipped,
    Failed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportEntry {
    pub kind: ResourceKind,
    pub path: String,
    pub status: FileStatus,
    pub size: u64,
    /// Skip reason or error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub schema: u32,
    pub files: Vec<ReportEntry>
}

impl Report {
    pub async fn save(&self, path: &Path) -> Result<(), CatalogError> {
        save_json(path, self).await
    }

    pub(crate) fn extend(&mut self, kind: ResourceKind, summaries: &[Summary]) {
        self.files.extend(summaries.iter().map(|summary| {
            let (status, message) = match &summary.status {
                DownloadStatus::Success => (FileStatus::Downloaded, None),
                DownloadStatus::Skipped(reason) => (FileStatus::Skipped, Some(reason.to_string())),
                DownloadStatus::Failed(error) | DownloadStatus::HashMismatch(error) => {
                    (FileStatus::Failed, Some(error.to_string()))
                }
                DownloadStatus::NotStarted => (FileStatus::Failed, Some("Not started".into()))
            };
            ReportEntry {
                kind,
                path: summary.download.filename.clone(),
                status,
                size: summary.size,
                message
            }
        }));
    }

    pub fn failures(&self) -> impl Iterator<Item = &ReportEntry> {
        self.files.iter().filter(|file| file.status == FileStatus::Failed)
    }
}

impl Default for Report {
    fn default() -> Self {
        Self {
            schema: MANIFEST_SCHEMA,
            files: Vec::new()
        }
    }
}

impl fmt::Display for ReportEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path, self.message.as_deref().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use baad_shared::{DownloadAsset, DownloadMedia, DownloadTable, Downloads, HashValue};
    use serde_json::json;

    use super::*;
    use crate::download::FilterMethod;
    use crate::error::FilterError;

    fn downloads() -> Downloads {
        Downloads {
            assets: vec![DownloadAsset {
                url: "https://cdn.test/Android_PatchPack/pack-1.zip".into(),
                path: "AssetBundles/pack-1.zip".into(),
                hash: HashValue::Crc(123),
                size: 100,
                bundle_files: vec!["ch0230-a.bundle".into(), "ch0231-b.bundle".into()]
            }],
            tables: vec![DownloadTable {
                url: "https://cdn.test/TableBundles/ExcelDB.db".into(),
                path: "TableBundles/ExcelDB.db".into(),
                hash: HashValue::Md5("755f85c2723bb39381c7379a604160d8".into()),
                size: 4,
                bundle_files: Vec::new()
            }],
            media: vec![DownloadMedia {
                url: "https://cdn.test/MediaResources/Audio/voice.ogg".into(),
                path: "MediaResources/Audio/voice.ogg".into(),
                hash: HashValue::Crc(7),
                size: 9
            }]
        }
    }

    fn entry(path: &str) -> ManifestEntry {
        ManifestEntry {
            kind: ResourceKind::Media,
            path: path.into(),
            url: "https://cdn.test/file".into(),
            size: Some(4),
            hash: Some("755f85c2723bb39381c7379a604160d8".into()),
            hash_type: Some(ManifestHash::Md5),
            member: None,
            members: Vec::new()
        }
    }

    fn manifest(resources: Vec<ManifestEntry>) -> Manifest {
        Manifest {
            schema: MANIFEST_SCHEMA,
            region: "japan".into(),
            platform: "android".into(),
            build: "standard".into(),
            version: Some("1.0.0".into()),
            source: "https://cdn.test".into(),
            resources
        }
    }

    #[test]
    fn plan_lists_packs_with_their_members() {
        let entries = plan(&downloads(), None);

        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0], ManifestEntry {
            kind: ResourceKind::Assets,
            path: "AssetBundles/pack-1.zip".into(),
            url: "https://cdn.test/Android_PatchPack/pack-1.zip".into(),
            size: Some(100),
            hash: Some("123".into()),
            hash_type: Some(ManifestHash::Crc32),
            member: None,
            members: vec!["ch0230-a.bundle".into(), "ch0231-b.bundle".into()]
        });
        assert_eq!(entries[1].kind, ResourceKind::Tables);
        assert_eq!(entries[1].hash_type, Some(ManifestHash::Md5));
        assert_eq!(entries[2].path, "MediaResources/Audio/voice.ogg");
    }

    #[test]
    fn filtered_plan_extracts_matching_members_without_archive_checksums() -> Result<(), FilterError>
    {
        let filter = ResourceFilter::new("ch0230-a.bundle", FilterMethod::Exact)?;
        let entries = plan(&downloads(), Some(&filter));

        assert_eq!(entries, vec![ManifestEntry {
            kind: ResourceKind::Assets,
            path: "AssetBundles/ch0230-a.bundle".into(),
            url: "https://cdn.test/Android_PatchPack/pack-1.zip".into(),
            size: None,
            hash: None,
            hash_type: None,
            member: Some("ch0230-a.bundle".into()),
            members: Vec::new()
        }]);
        Ok(())
    }

    #[test]
    fn entries_round_trip_through_downloads() -> Result<(), ManifestError> {
        for original in plan(&downloads(), None) {
            let download = original.to_download()?;
            let packed = original.members.clone();
            assert_eq!(ManifestEntry::from_download(original.kind, &download, &packed), original);
        }
        Ok(())
    }

    #[test]
    fn rejects_paths_that_could_leave_the_output_directory() {
        for path in [
            "",
            "/etc/passwd",
            "../escape",
            "a/../../escape",
            "a//b",
            "./a",
            "a\\..\\b",
            "C:/Windows",
            "a/"
        ] {
            assert!(
                matches!(entry(path).to_download(), Err(ManifestError::UnsafePath(_))),
                "{path:?} was accepted"
            );
        }
        assert!(entry("MediaResources/Audio/voice.ogg").to_download().is_ok());
    }

    #[test]
    fn rejects_hashes_without_algorithm_and_non_http_urls() {
        let mut untyped = entry("a.bin");
        untyped.hash_type = None;
        assert!(matches!(untyped.to_download(), Err(ManifestError::MissingHashType(_))));

        let mut local = entry("a.bin");
        local.url = "file:///etc/passwd".into();
        assert!(matches!(local.to_download(), Err(ManifestError::InvalidUrl { .. })));

        let mut member = entry("a.bin");
        member.member = Some(String::new());
        assert!(matches!(member.to_download(), Err(ManifestError::InvalidMember(_))));
    }

    #[test]
    fn validate_checks_schema_region_and_conflicting_paths() {
        assert!(manifest(vec![entry("a.bin"), entry("a.bin")]).validate("japan").is_ok());

        let mut other = entry("a.bin");
        other.size = Some(5);
        assert!(matches!(
            manifest(vec![entry("a.bin"), other]).validate("japan"),
            Err(ManifestError::DuplicatePath(_))
        ));
        assert!(matches!(
            manifest(vec![entry("a.bin")]).validate("global"),
            Err(ManifestError::RegionMismatch { .. })
        ));

        let mut future = manifest(Vec::new());
        future.schema = MANIFEST_SCHEMA + 1;
        assert!(matches!(future.validate("japan"), Err(ManifestError::UnsupportedSchema(_))));
    }

    #[test]
    fn json_format_is_stable() -> Result<(), serde_json::Error> {
        let mut member = entry("AssetBundles/a.bundle");
        member.kind = ResourceKind::Assets;
        member.size = None;
        member.hash = None;
        member.hash_type = None;
        member.member = Some("a.bundle".into());

        assert_eq!(
            serde_json::to_value(manifest(vec![entry("a.bin"), member]))?,
            json!({
                "schema": 1,
                "region": "japan",
                "platform": "android",
                "build": "standard",
                "version": "1.0.0",
                "source": "https://cdn.test",
                "resources": [
                    {
                        "kind": "media",
                        "path": "a.bin",
                        "url": "https://cdn.test/file",
                        "size": 4,
                        "hash": "755f85c2723bb39381c7379a604160d8",
                        "hash_type": "md5"
                    },
                    {
                        "kind": "assets",
                        "path": "AssetBundles/a.bundle",
                        "url": "https://cdn.test/file",
                        "member": "a.bundle"
                    }
                ]
            })
        );
        Ok(())
    }
}
