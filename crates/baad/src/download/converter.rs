use std::collections::HashSet;
use std::path::Path;

use baad_dm::{Download, HashType};
use baad_shared::{DownloadAsset, DownloadMedia, DownloadTable, HashValue, PackedFile};
use baad_utils::file::filename_matches;
use fastcat::fconcat;
use reqwest::Url;

use crate::download::ResourceFilter;

pub fn convert_assets(assets: &[DownloadAsset], filter: Option<&ResourceFilter>) -> Vec<Download> {
    let mut downloads = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();

    for asset in assets {
        if let Some(f) = filter
            && let Some(filename) = Path::new(&asset.path).file_name().and_then(|n| n.to_str())
            && f.matches(filename)
        {
            try_add(&mut seen, &mut downloads, asset);
            continue;
        }

        if let Some(f) = filter {
            for bundle in &asset.bundle_files {
                if f.matches(&bundle.name)
                    && seen.insert(bundle.name.as_str())
                    && let Some(dl) = create_member_download(asset, bundle)
                {
                    downloads.push(dl);
                }
            }
        } else {
            try_add(&mut seen, &mut downloads, asset);
        }
    }

    downloads
}

pub fn convert_tables(tables: &[DownloadTable], filter: Option<&ResourceFilter>) -> Vec<Download> {
    tables
        .iter()
        .filter(|t| {
            filter.is_none_or(|f| filename_matches(&t.path, |filename| f.matches(filename)))
        })
        .filter_map(|t| create_download(&t.url, &t.path, &t.hash, t.size))
        .collect()
}

pub fn convert_media(media: &[DownloadMedia], filter: Option<&ResourceFilter>) -> Vec<Download> {
    media
        .iter()
        .filter(|media| media_matches(&media.path, filter))
        .filter_map(|media| create_download(&media.url, &media.path, &media.hash, media.size))
        .collect()
}

fn media_matches(path: &str, filter: Option<&ResourceFilter>) -> bool {
    filter.is_none_or(|f| filename_matches(path, |filename| f.matches(filename)))
}

fn try_add<'a>(
    seen: &mut HashSet<&'a str>,
    downloads: &mut Vec<Download>,
    asset: &'a DownloadAsset
) {
    if seen.insert(asset.path.as_str())
        && let Some(dl) = create_download(&asset.url, &asset.path, &asset.hash, asset.size)
    {
        downloads.push(dl);
    }
}

fn convert_path_to_bundle(zip_path: &str, bundle_filename: &str) -> String {
    zip_path.rfind('/').map_or_else(
        || bundle_filename.into(),
        |last_slash| fconcat!(&zip_path[..last_slash], "/", bundle_filename)
    )
}

/// Members are verified against the archive's central directory; the
/// catalog's size and checksum of the extracted file are carried along for
/// callers, since the archive's own checks do not apply to it.
fn create_member_download(asset: &DownloadAsset, bundle: &PackedFile) -> Option<Download> {
    let parsed_url = Url::parse(&asset.url).ok()?;
    Some(
        Download::builder()
            .url(parsed_url)
            .filename(convert_path_to_bundle(&asset.path, &bundle.name))
            .maybe_hash(bundle.hash.as_ref().map(HashValue::as_string))
            .maybe_hash_type(bundle.hash.as_ref().map(hash_type))
            .target_file(bundle.name.clone())
            .maybe_size(bundle.size.and_then(|size| u64::try_from(size).ok()))
            .build()
    )
}

const fn hash_type(hash: &HashValue) -> HashType {
    match hash {
        HashValue::Crc(_) => HashType::Crc32,
        HashValue::Md5(_) => HashType::Md5
    }
}

fn create_download(url: &str, path: &str, hash: &HashValue, size: i64) -> Option<Download> {
    let parsed_url = Url::parse(url).ok()?;
    Some(
        Download::builder()
            .url(parsed_url)
            .filename(path.into())
            .hash(hash.as_string())
            .hash_type(hash_type(hash))
            .maybe_size(u64::try_from(size).ok().filter(|s| *s > 0))
            .build()
    )
}
