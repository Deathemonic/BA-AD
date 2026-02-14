use crate::strategy::{AssetDownload, MediaDownload, TableDownload};

pub struct Downloads {
    pub assets: Vec<AssetDownload>,
    pub tables: Vec<TableDownload>,
    pub media: Vec<MediaDownload>
}
