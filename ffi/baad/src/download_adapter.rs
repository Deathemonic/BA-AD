use baad_native::download::ResourceCategory;

pub fn category_bits(assets: bool, tables: bool, media: bool) -> u8 {
    ResourceCategory::new()
        .include_if(assets, ResourceCategory::Assets)
        .include_if(tables, ResourceCategory::Tables)
        .include_if(media, ResourceCategory::Media)
        .bits()
}

pub const fn resource_category(bits: u8) -> ResourceCategory {
    ResourceCategory::from_bits_truncate(bits).or_all_if_empty()
}
