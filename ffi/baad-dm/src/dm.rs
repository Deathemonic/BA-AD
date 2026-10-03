#[diplomat::bridge]
pub mod ffi {
    use std::fmt::Write;
    use std::path::Path;

    use baad_shared_ffi::error::ffi::BaadError;
    use baad_shared_ffi::models::ffi::BaadSharedStrings;
    use baad_shared_ffi::progress::ffi::BaadSharedProgressStatusKind;
    use baad_utils_ffi::utils::ffi::BaadUtilsBytes;

    pub enum BaadDmHashType {}
    #[diplomat::opaque]
    pub struct BaadDmDownloads(pub Vec<baad_dm::Download>);
    impl BaadDmDownloads {
        pub fn new() -> Box<Self> { Box::new(Self(Vec::new())) }

        pub fn push(
            &mut self,
            url: &str,
            filename: &str,
            hash: &str,
            target_file: &str,
            size: Option<u64>
        ) -> Result<(), Box<BaadError>> {
            let mut item =
                baad_dm::Download::try_from(url).map_err(baad_shared_ffi::error::error)?;
            if !filename.is_empty() {
                item.filename = filename.into();
            }
            item.hash = (!hash.is_empty()).then(|| hash.to_owned());
            item.target_file = (!target_file.is_empty()).then(|| target_file.to_owned());
            item.size = size;
            self.0.push(item);
            Ok(())
        }

        pub const fn is_empty(&self) -> bool { self.0.is_empty() }

        pub const fn len(&self) -> usize { self.0.len() }
    }
    #[derive(Clone, Copy)]
    pub struct BaadDmDownloaderConfig {}
    impl BaadDmDownloaderConfig {
        pub fn default_config() -> Self {
            baad_dm::DownloaderConfig::builder().directory(Path::new("")).build().into()
        }
    }
    #[diplomat::opaque]
    pub struct BaadDm;
    impl BaadDm {
        pub fn detect_hash_type(hash: &str) -> Option<BaadDmHashType> {
            baad_dm::detect_hash_type(hash).map(Into::into)
        }

        pub fn verify_hash(path: &str, hash: &str) -> Result<bool, Box<BaadError>> {
            baad_dm::verify_hash(
                Path::new(path),
                (!hash.is_empty()).then(|| hash.to_owned()).as_ref()
            )
            .map_err(baad_shared_ffi::error::error)
        }

        pub fn range_header(start: u64, end: Option<u64>, output: &mut DiplomatWrite) {
            let _ = output.write_str(&baad_dm::client::create_range_header(start, end));
        }

        pub fn resolve_url(url: &str, output: &mut DiplomatWrite) -> Result<(), Box<BaadError>> {
            let value = baad_shared_ffi::error::blocking(crate::adapter::resolve_url(url))?;
            let _ = output.write_str(&value);
            Ok(())
        }

        pub fn download(
            config: BaadDmDownloaderConfig,
            directory: &str,
            proxy: &str,
            items: &mut BaadDmDownloads
        ) -> Result<Box<BaadDmSummaries>, Box<BaadError>> {
            crate::adapter::validate_limits(
                config.concurrent_downloads,
                config.max_chunks_per_file,
                config.max_concurrent_chunks
            )
            .map_err(baad_shared_ffi::error::error)?;
            let proxy = crate::adapter::create_proxy((!proxy.is_empty()).then_some(proxy))
                .map_err(baad_shared_ffi::error::error)?;
            let downloader = baad_dm::Downloader::new(config.native(Path::new(directory), proxy));
            let downloads = std::mem::take(&mut items.0);
            let values = baad_shared_ffi::runtime::block_on(downloader.download(&downloads))
                .map_err(baad_shared_ffi::error::runtime_error)?;
            Ok(Box::new(BaadDmSummaries(values)))
        }

        pub fn zip_extract_file(
            url: &str,
            target: &str
        ) -> Result<Box<BaadUtilsBytes>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(crate::adapter::zip_extract_file(url, target))
                .map(|value| Box::new(BaadUtilsBytes(value.into())))
        }

        pub fn zip_index(url: &str) -> Result<Box<BaadDmZipIndex>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(crate::adapter::zip_index(url))
                .map(|value| Box::new(BaadDmZipIndex(value)))
        }
    }
    #[diplomat::opaque]
    pub struct BaadDmZipIndex(pub baad_dm::ZipIndex);
    impl BaadDmZipIndex {
        pub fn names(&self) -> Box<BaadSharedStrings> {
            Box::new(BaadSharedStrings(self.0.names().map(str::to_owned).collect()))
        }

        pub fn get(&self, name: &str) -> Option<BaadDmZipFileInfo> {
            self.0.get(name).cloned().map(Into::into)
        }
    }
    #[derive(Clone, Copy)]
    pub struct BaadDmZipFileInfo {}
    #[diplomat::opaque]
    pub struct BaadDmSummaries(pub Vec<baad_dm::Summary>);
    impl BaadDmSummaries {
        pub const fn is_empty(&self) -> bool { self.0.is_empty() }

        pub const fn len(&self) -> usize { self.0.len() }

        pub fn get<'a>(&'a self, index: usize) -> Option<&'a BaadDmSummary> {
            self.0.get(index).map(BaadDmSummary::borrow)
        }
    }
    #[diplomat::opaque]
    #[repr(transparent)]
    pub struct BaadDmSummary(pub baad_dm::Summary);
    impl BaadDmSummary {
        pub fn url<'a>(&'a self) -> &'a str { self.0.download.url.as_str() }

        pub fn filename<'a>(&'a self) -> &'a str { &self.0.download.filename }

        pub const fn size(&self) -> u64 { self.0.size }

        pub const fn status_code(&self) -> u16 { self.0.status_code.as_u16() }

        pub const fn resumable(&self) -> bool { self.0.resumable }

        pub const fn is_success(&self) -> bool { self.0.is_success() }

        pub fn status(&self) -> BaadSharedProgressStatusKind { (&self.0.status).into() }

        pub fn reason<'a>(&'a self) -> &'a str {
            baad_shared_ffi::progress::status_reason(&self.0.status)
        }
    }
}
impl ffi::BaadDmSummary {
    const fn borrow(value: &baad_dm::Summary) -> &Self {
        unsafe { &*(std::ptr::from_ref(value).cast::<Self>()) }
    }
}
const _: () = {
    assert!(std::mem::size_of::<ffi::BaadDmSummary>() == std::mem::size_of::<baad_dm::Summary>());
    assert!(std::mem::align_of::<ffi::BaadDmSummary>() == std::mem::align_of::<baad_dm::Summary>());
};
