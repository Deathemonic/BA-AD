use crate::strategy::{Asset, Media, Table};

pub struct Downloads {
    pub assets: Vec<Asset>,
    pub tables: Vec<Table>,
    pub media: Vec<Media>
}
