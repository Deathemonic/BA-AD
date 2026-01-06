use crate::helpers::{HashValue, TableCatalog};

pub struct TableDownload {
    pub url: String,
    pub path: String,
    pub hash: HashValue,
    pub size: i64,
}

pub struct TableStrategy;

impl TableStrategy {
    pub fn build_downloads(catalog: &TableCatalog, catalog_url: &str) -> Vec<TableDownload> {
        catalog
            .table
            .values()
            .map(|entry| TableDownload {
                url: format!("{}/TableBundles/{}", catalog_url, entry.name),
                path: format!("TableBundles/{}", entry.name),
                hash: HashValue::Crc(entry.crc),
                size: entry.size,
            })
            .collect()
    }
}

