use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[allow(non_upper_case_globals)]
    pub struct ResourceCategory: u8 {
        const Assets = 1 << 0;
        const Tables = 1 << 1;
        const Media = 1 << 2;
        const ALL = Self::Assets.bits() | Self::Tables.bits() | Self::Media.bits();
    }
}

impl ResourceCategory {
    pub const fn new() -> Self { Self::empty() }

    pub fn include_if(self, enabled: bool, category: Self) -> Self {
        if enabled { self | category } else { self }
    }

    pub const fn or_all_if_empty(self) -> Self { if self.is_empty() { Self::ALL } else { self } }
}

impl Default for ResourceCategory {
    fn default() -> Self { Self::new() }
}

impl<const N: usize> From<[ResourceCategory; N]> for ResourceCategory {
    fn from(categories: [ResourceCategory; N]) -> Self { categories.into_iter().collect() }
}

impl From<Vec<ResourceCategory>> for ResourceCategory {
    fn from(categories: Vec<ResourceCategory>) -> Self { categories.into_iter().collect() }
}
