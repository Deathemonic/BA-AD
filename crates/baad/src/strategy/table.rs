use baad_core::{HashValue, TableCatalog};

pub struct Table {
    pub url: String,
    pub path: String,
    pub hash: HashValue,
    pub size: i64
}

pub struct TableStrategy;

impl TableStrategy {
    pub fn build_downloads(catalog: &TableCatalog, catalog_url: &str) -> Vec<Table> {
        catalog
            .table
            .values()
            .map(|entry| Table {
                url: format!("{}/TableBundles/{}", catalog_url, entry.name),
                path: format!("TableBundles/{}", entry.name),
                hash: HashValue::Crc(entry.crc),
                size: entry.size
            })
            .collect()
    }
}
