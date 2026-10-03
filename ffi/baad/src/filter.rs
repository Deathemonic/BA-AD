#[diplomat::bridge]
pub mod ffi {
    use baad_shared_ffi::error::ffi::BaadError;

    pub enum BaadFilterMethod {}

    #[diplomat::opaque]
    pub struct BaadResourceFilter(pub baad_native::download::ResourceFilter);

    impl BaadResourceFilter {
        pub fn new(pattern: &str, method: BaadFilterMethod) -> Result<Box<Self>, Box<BaadError>> {
            baad_native::download::ResourceFilter::new(pattern, method.into())
                .map(|filter| Box::new(Self(filter)))
                .map_err(baad_shared_ffi::error::error)
        }

        pub fn matches(&self, path: &str) -> bool { self.0.matches(path) }
    }
}
