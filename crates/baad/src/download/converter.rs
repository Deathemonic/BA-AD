use std::collections::HashSet;
use std::path::Path;

use baad_dm::Download;
use baad_shared::{DownloadAsset, DownloadMedia, DownloadTable, HashValue};
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
            for bundle_name in &asset.bundle_files {
                if f.matches(bundle_name)
                    && seen.insert(bundle_name.as_str())
                    && let Some(dl) = create_download(
                        &asset.url,
                        &convert_path_to_bundle(&asset.path, bundle_name),
                        &asset.hash,
                        Some(bundle_name)
                    )
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
        .filter_map(|t| create_download(&t.url, &t.path, &t.hash, None))
        .collect()
}

pub fn convert_media(media: &[DownloadMedia], filter: Option<&ResourceFilter>) -> Vec<Download> {
    media
        .iter()
        .filter(|media| media_matches(&media.path, filter))
        .filter_map(|media| {
            create_download(&media.url, &media.path, &media.hash, media.target.as_deref())
        })
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
        && let Some(dl) = create_download(&asset.url, &asset.path, &asset.hash, None)
    {
        downloads.push(dl);
    }
}

fn convert_path_to_bundle(zip_path: &str, bundle_filename: &str) -> String {
    if let Some(last_slash) = zip_path.rfind('/') {
        fconcat!(&zip_path[..last_slash], "/", bundle_filename)
    } else {
        bundle_filename.into()
    }
}

fn create_download(
    url: &str,
    path: &str,
    hash: &HashValue,
    target: Option<&str>
) -> Option<Download> {
    let parsed_url = Url::parse(url).ok()?;
    Some(
        Download::builder()
            .url(parsed_url)
            .filename(path.into())
            .hash(hash.as_string())
            .maybe_target_file(target.map(|s| s.into()))
            .build()
    )
}
