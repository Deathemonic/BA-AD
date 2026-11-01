#[diplomat::bridge]
pub mod ffi {
    use std::mem::replace;
    use std::path::{Path, PathBuf};

    use baad_shared_ffi::error::ffi::BaadError;
    use baad_shared_ffi::models::ffi::BaadSharedDownloads;

    use crate::download_adapter::category_bits;
    use crate::filter::ffi::BaadResourceFilter;

    #[derive(Clone, Copy)]
    pub struct BaadDownloaderOptions {}

    #[diplomat::opaque]
    pub struct BaadResourceDownloader;

    impl BaadResourceDownloader {
        pub fn category(assets: bool, tables: bool, media: bool) -> u8 {
            category_bits(assets, tables, media)
        }

        pub fn download(
            options: BaadDownloaderOptions,
            output_dir: &str,
            proxy: &str,
            downloads: &mut BaadSharedDownloads,
            filter: Option<&BaadResourceFilter>
        ) -> Result<(), Box<BaadError>> {
            baad_dm_ffi::adapter::validate_limits(
                options.limit,
                options.max_chunks_per_file,
                options.max_concurrent_chunks
            )
            .map_err(baad_shared_ffi::error::error)?;
            baad_dm_ffi::adapter::create_proxy((!proxy.is_empty()).then_some(proxy))
                .map_err(baad_shared_ffi::error::error)?;
            let downloader = options
                .native(PathBuf::from(output_dir), (!proxy.is_empty()).then(|| proxy.to_owned()));
            let downloads = replace(&mut downloads.0, baad_shared::Downloads {
                assets: Vec::new(),
                tables: Vec::new(),
                media: Vec::new()
            });
            baad_shared_ffi::error::blocking(
                downloader.download(downloads, filter.map(|filter| &filter.0))
            )
        }

        pub fn download_file(
            url: &str,
            output_path: &str,
            hash: &str,
            retries: u32
        ) -> Result<(), Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_native::download::download_file(
                url,
                Path::new(output_path),
                (!hash.is_empty()).then(|| hash.to_owned()),
                retries
            ))
        }
    }
}
