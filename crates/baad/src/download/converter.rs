use std::collections::HashSet;
use std::path::Path;

use baad_core::{DownloadAsset, DownloadMedia, DownloadTable, HashValue};
use baad_dm::Download;
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
            if seen.insert(asset.path.as_str())
                && let Some(dl) = create_download(&asset.url, &asset.path, &asset.hash, None)
            {
                downloads.push(dl);
            }
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
        } else if seen.insert(asset.path.as_str())
            && let Some(dl) = create_download(&asset.url, &asset.path, &asset.hash, None)
        {
            downloads.push(dl);
        }
    }

    downloads
}

pub fn convert_tables(tables: &[DownloadTable], filter: Option<&ResourceFilter>) -> Vec<Download> {
    tables
        .iter()
        .filter(|t| {
            filter.is_none_or(|f| {
                Path::new(&t.path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|filename| f.matches(filename))
            })
        })
        .filter_map(|t| create_download(&t.url, &t.path, &t.hash, None))
        .collect()
}

pub fn convert_media(media: &[DownloadMedia], filter: Option<&ResourceFilter>) -> Vec<Download> {
    media
        .iter()
        .filter(|m| {
            filter.is_none_or(|f| {
                Path::new(&m.path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|filename| f.matches(filename))
            })
        })
        .filter_map(|m| create_download(&m.url, &m.path, &m.hash, None))
        .collect()
}

fn convert_path_to_bundle(zip_path: &str, bundle_filename: &str) -> String {
    if let Some(last_slash) = zip_path.rfind('/') {
        format!("{}/{}", &zip_path[..last_slash], bundle_filename)
    } else {
        bundle_filename.to_string()
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
            .filename(path.to_string())
            .hash(hash.as_string())
            .maybe_target_file(target.map(|s| s.to_string()))
            .build()
    )
}
