pub fn category_bits(assets: bool, tables: bool, media: bool) -> u8 {
    baad_native::download::ResourceCategory::new()
        .include_if(assets, baad_native::download::ResourceCategory::Assets)
        .include_if(tables, baad_native::download::ResourceCategory::Tables)
        .include_if(media, baad_native::download::ResourceCategory::Media)
        .bits()
}

pub const fn resource_category(bits: u8) -> baad_native::download::ResourceCategory {
    baad_native::download::ResourceCategory::from_bits_truncate(bits).or_all_if_empty()
}
