use crate::helpers::{
    ASSET_APK, ApkError, CONFIG_APK, DATA_PATH, DATA_PATTERN, GLOBAL_DATA_APK, JP_DATA_APK,
    LIBIL2CPP_PATH, LIBIL2CPP_PATTERN, METADATA_PATH, METADATA_PATTERN, ServerConfig, ServerRegion,
};

use baad_core::{file, info};
use glob::Pattern;
use std::fs::{self, File};
use std::io::{self, BufWriter, Cursor, Read};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use zip::ZipArchive;

pub struct ExtractionRule<'a> {
    pub apk: &'a str,
    pub path: &'a [&'a str],
    pub pattern: &'a str,
    pub output: Box<Path>,
}

pub struct ApkExtractor {
    config: Rc<ServerConfig>,
}

impl ApkExtractor {
    pub fn new(config: Rc<ServerConfig>) -> Result<Self, ApkError> {
        Ok(Self { config })
    }

    pub fn extract(&self, rule: ExtractionRule) -> Result<(), ApkError> {
        info!("Extracting apk...");
        let apk_path = file::get_data_path(&self.config.apk_path)?;

        let file = File::open(&apk_path)?;
        let mut archive = ZipArchive::new(file)?;

        let mut target_apk = archive.by_name(rule.apk)?;
        let size_hint = target_apk.size() as usize;
        let mut buf = Vec::with_capacity(size_hint);
        target_apk.read_to_end(&mut buf)?;

        let cursor = Cursor::new(buf);
        let mut inner_archive = ZipArchive::new(cursor)?;

        fs::create_dir_all(&rule.output)?;

        for i in 0..inner_archive.len() {
            let mut file = inner_archive.by_index(i)?;
            let file_path = PathBuf::from(file.name());

            if self.matches_rule(&file_path, &rule)?
                && let Some(file_name) = file_path.file_name()
            {
                let out = rule.output.join(file_name);

                let outfile = File::create(&out)?;
                let mut writer = BufWriter::with_capacity(64 * 1024, outfile);
                io::copy(&mut file, &mut writer)?;
            }
        }

        Ok(())
    }

    fn matches_rule(&self, file_path: &Path, rule: &ExtractionRule) -> Result<bool, ApkError> {
        let mut components = file_path.components();

        for target in rule.path {
            match components.next() {
                Some(component) => {
                    if component.as_os_str().to_string_lossy() != *target {
                        return Ok(false);
                    }
                }
                None => return Ok(false),
            }
        }

        let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let pattern = Pattern::new(rule.pattern)?;
        Ok(pattern.matches(file_name))
    }

    pub fn extract_data(&self) -> Result<(), ApkError> {
        info!("Extracting game data...");

        let rule = ExtractionRule {
            apk: match self.config.region {
                ServerRegion::Global => GLOBAL_DATA_APK,
                ServerRegion::Japan => ASSET_APK,
            },
            path: DATA_PATH,
            pattern: DATA_PATTERN,
            output: file::get_data_path("data")?.into_boxed_path(),
        };

        self.extract(rule)
    }

    pub fn extract_il2cpp(&self) -> Result<(), ApkError> {
        let il2cpp_path: PathBuf = match self.config.region {
            ServerRegion::Global => file::get_data_path("il2cpp/global")?,
            ServerRegion::Japan => file::get_data_path("il2cpp/japan")?,
        };

        info!("Extracting IL2CPP files...");

        let lib_rule = ExtractionRule {
            apk: CONFIG_APK,
            path: LIBIL2CPP_PATH,
            pattern: LIBIL2CPP_PATTERN,
            output: il2cpp_path.clone().into_boxed_path(),
        };
        self.extract(lib_rule)?;

        let metadata_rule = ExtractionRule {
            apk: match self.config.region {
                ServerRegion::Global => GLOBAL_DATA_APK,
                ServerRegion::Japan => JP_DATA_APK,
            },
            path: METADATA_PATH,
            pattern: METADATA_PATTERN,
            output: il2cpp_path.into_boxed_path(),
        };
        self.extract(metadata_rule)?;

        Ok(())
    }
}
