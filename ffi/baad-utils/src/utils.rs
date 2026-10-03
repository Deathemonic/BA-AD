#[diplomat::bridge]
pub mod ffi {
    use std::fmt::Write;
    use std::path::{Path, PathBuf};

    use baad_shared_ffi::error::ffi::BaadError;
    use baad_shared_ffi::progress::ffi::BaadSharedProgressEvent;
    #[derive(Clone, Copy)]
    pub struct BaadUtilsLoggingConfig {}
    impl BaadUtilsLoggingConfig {
        pub fn default_config() -> Self { baad_utils::config::LoggingConfig::default().into() }
    }
    pub struct BaadUtilsAlignedLine<'a> {}
    pub enum BaadUtilsLogLevel {
        Trace,
        Debug,
        Info,
        Warn,
        Error
    }
    #[diplomat::opaque]
    pub struct BaadUtils;
    impl BaadUtils {
        pub fn contains_url(value: &str) -> bool {
            baad_utils::formatter::styles::contains_url(value)
        }

        pub fn format_urls(
            value: &str,
            text_style: &BaadUtilsStyle,
            url_style: &BaadUtilsStyle,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BaadError>> {
            use owo_colors::OwoColorize;
            baad_utils::formatter::styles::format_urls(
                value,
                output,
                |writer, text| write!(writer, "{}", text.style(text_style.0)),
                |writer, url| write!(writer, "{}", url.style(url_style.0))
            )
            .map_err(baad_shared_ffi::error::error)
        }

        pub fn init_logging(c: BaadUtilsLoggingConfig) -> Result<(), Box<BaadError>> {
            baad_utils::config::init_logging(c.into()).map_err(baad_shared_ffi::error::error)
        }

        pub fn flush_logs() { baad_utils::flush_logs(); }

        pub fn set_app_name(name: &str) -> Result<(), Box<BaadError>> {
            baad_utils::file::set_app_name(name).map_err(baad_shared_ffi::error::error)
        }

        pub fn set_data_dir(path: &str) -> Result<(), Box<BaadError>> {
            baad_utils::file::set_data_dir(PathBuf::from(path))
                .map_err(baad_shared_ffi::error::error)
        }

        pub fn data_dir(output: &mut DiplomatWrite) -> Result<(), Box<BaadError>> {
            let path = baad_utils::file::data_dir().map_err(baad_shared_ffi::error::error)?;
            let _ = output.write_str(&path.to_string_lossy());
            Ok(())
        }

        pub fn get_data_path(
            filename: &str,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BaadError>> {
            let path =
                baad_utils::file::get_data_path(filename).map_err(baad_shared_ffi::error::error)?;
            let _ = output.write_str(&path.to_string_lossy());
            Ok(())
        }

        pub fn filename_or<'a>(path: &'a str) -> &'a str { baad_utils::file::filename_or(path) }

        pub fn load_file(path: &str) -> Result<Box<BaadUtilsBytes>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_utils::file::load_file(Path::new(path)))
                .map(|bytes| Box::new(BaadUtilsBytes(bytes.into())))
        }

        pub fn save_file(path: &str, bytes: &[u8]) -> Result<(), Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_utils::file::save_file(Path::new(path), bytes))
        }

        pub fn json_save(path: &str, json: &str) -> Result<(), Box<BaadError>> {
            baad_shared_ffi::error::blocking(crate::adapter::json_save_string(
                Path::new(path),
                json
            ))
        }

        pub fn get_output_dir(
            path: &str,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BaadError>> {
            let path = (!path.is_empty()).then(|| Path::new(path));
            let value = baad_shared_ffi::error::blocking(baad_utils::file::get_output_dir(path))?;
            let _ = output.write_str(&value.to_string_lossy());
            Ok(())
        }

        pub fn progress_started(id: &str, label: &str, count_unit: bool, total: u64) {
            crate::adapter::progress_started(
                id,
                label,
                if count_unit {
                    baad_shared::ProgressUnit::Count
                } else {
                    baad_shared::ProgressUnit::Bytes
                },
                total
            );
        }

        pub fn progress_advance(id: &str, current: u64, total: u64) {
            crate::adapter::progress_advance(id, current, total);
        }

        pub fn progress_completed(id: &str) { crate::adapter::progress_completed(id); }

        pub fn progress_failed(id: &str, reason: &str) {
            crate::adapter::progress_failed(id, reason);
        }

        pub fn log(level: BaadUtilsLogLevel, success: bool, message: &str) {
            crate::adapter::log_message(level, success, message, None);
        }

        pub fn log_with_field(
            level: BaadUtilsLogLevel,
            success: bool,
            message: &str,
            name: &str,
            value: &str
        ) {
            let value = crate::adapter::render_fields(&[(name, value)]);
            crate::adapter::log_message(level, success, message, value.as_deref());
        }

        pub fn log_with_fields(
            level: BaadUtilsLogLevel,
            success: bool,
            message: &str,
            names: &[DiplomatStrSlice],
            values: &[DiplomatStrSlice]
        ) -> Result<(), Box<BaadError>> {
            let fields =
                crate::adapter::fields(names, values).map_err(baad_shared_ffi::error::error)?;
            let rendered = crate::adapter::render_fields(&fields);
            crate::adapter::log_message(level, success, message, rendered.as_deref());
            Ok(())
        }

        pub fn format_bytes(value: u64, output: &mut DiplomatWrite) {
            let _ = write!(output, "{}", baad_utils::formatter::HumanBytes(value));
        }

        pub fn terminal_is_terminal() -> bool { baad_utils::progress::terminal::is_terminal() }

        pub fn terminal_size() -> BaadUtilsTerminalSize {
            let (width, height) = baad_utils::progress::terminal::size().unwrap_or((0, 0));
            BaadUtilsTerminalSize {
                width: width as u64,
                height: height as u64
            }
        }

        pub fn create_parent_dir(path: &str) -> Result<(), Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_utils::file::create_parent_dir(Path::new(path)))
        }

        pub fn is_dir_empty(path: &str) -> Result<bool, Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_utils::file::is_dir_empty(Path::new(path)))
        }

        pub fn clear_all(path: &str) -> Result<(), Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_utils::file::clear_all(Path::new(path)))
        }

        pub fn json_load(path: &str, output: &mut DiplomatWrite) -> Result<(), Box<BaadError>> {
            let value = baad_shared_ffi::error::blocking(crate::adapter::json_load_string(
                Path::new(path)
            ))?;
            let _ = output.write_str(&value);
            Ok(())
        }

        pub fn fetch_version(path: &str, output: &mut DiplomatWrite) -> Result<(), Box<BaadError>> {
            let value = baad_shared_ffi::error::blocking(baad_utils::network::fetch_version(path))?;
            let _ = output.write_str(&value);
            Ok(())
        }
    }
    #[diplomat::opaque]
    pub struct BaadUtilsBytes(pub baad_shared_ffi::adapter::ByteBuffer);
    impl BaadUtilsBytes {
        pub fn data<'a>(&'a self) -> &'a [u8] { self.0.as_ref() }
    }
    #[derive(Clone, Copy)]
    pub struct BaadUtilsTerminalSize {
        pub width: u64,
        pub height: u64
    }
    impl BaadUtils {
        pub fn filename_matches(path: &str, matches: impl Fn(&str) -> bool) -> bool {
            baad_utils::file::filename_matches(path, matches)
        }

        pub fn json_update(
            path: &str,
            updater: impl Fn(&mut BaadUtilsJson)
        ) -> Result<(), Box<BaadError>> {
            baad_shared_ffi::error::blocking(baad_utils::json::update::<serde_json::Value, _>(
                Path::new(path),
                |value| {
                    let mut json = BaadUtilsJson(std::mem::take(value));
                    updater(&mut json);
                    *value = json.0;
                }
            ))
        }

        pub fn create_proxy(url: &str) -> Result<Option<Box<BaadUtilsProxy>>, Box<BaadError>> {
            baad_utils::network::create_proxy((!url.is_empty()).then_some(url))
                .map(|proxy| proxy.map(|proxy| Box::new(BaadUtilsProxy(proxy))))
                .map_err(baad_shared_ffi::error::error)
        }

        pub fn fetch_response(url: &str) -> Result<Box<BaadUtilsResponse>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(async { baad_shared::client().get(url).send().await })
                .map(|response| Box::new(BaadUtilsResponse(Some(response))))
        }

        pub fn run(job: impl Fn() -> bool) {
            baad_utils::run(|| {
                if job() { Ok(()) } else { Err(eyre::eyre!("Foreign operation failed")) }
            });
        }

        pub fn run_async(job: impl Fn() -> bool) -> Result<(), Box<BaadError>> {
            baad_shared_ffi::runtime::block_on(baad_utils::run_async(|| async {
                if job() { Ok(()) } else { Err(eyre::eyre!("Foreign operation failed")) }
            }))
            .map_err(baad_shared_ffi::error::runtime_error)
        }

        pub fn log_recoverable_error(message: &str, recovery_action: &str) {
            baad_utils::log_recoverable_error(
                &eyre::Report::msg(message.to_owned()),
                recovery_action
            );
        }

        pub fn init_logging_with_model(
            config: BaadUtilsLoggingConfig,
            display: &mut BaadUtilsProgressDisplay
        ) -> Result<(), Box<BaadError>> {
            let model = display.0.take().ok_or_else(|| {
                baad_shared_ffi::error::error("Progress display already consumed")
            })?;
            baad_utils::config::init_logging_with_model(config.into(), model)
                .map_err(baad_shared_ffi::error::error)
        }
    }
    #[diplomat::opaque]
    pub struct BaadUtilsJson(pub serde_json::Value);
    impl BaadUtilsJson {
        pub fn parse(json: &str) -> Result<Box<Self>, Box<BaadError>> {
            serde_json::from_str(json)
                .map(|value| Box::new(Self(value)))
                .map_err(baad_shared_ffi::error::error)
        }

        pub fn replace(&mut self, json: &str) -> Result<(), Box<BaadError>> {
            self.0 = serde_json::from_str(json).map_err(baad_shared_ffi::error::error)?;
            Ok(())
        }

        pub fn write(&self, output: &mut DiplomatWrite) -> Result<(), Box<BaadError>> {
            let json = serde_json::to_string(&self.0).map_err(baad_shared_ffi::error::error)?;
            output.write_str(&json).map_err(baad_shared_ffi::error::error)
        }
    }
    #[diplomat::opaque]
    pub struct BaadUtilsProxy(pub reqwest::Proxy);
    impl BaadUtilsProxy {
        pub fn fetch_response(&self, url: &str) -> Result<Box<BaadUtilsResponse>, Box<BaadError>> {
            baad_shared_ffi::error::blocking(crate::adapter::fetch_with_proxy(&self.0, url))
                .map(|response| Box::new(BaadUtilsResponse(Some(response))))
        }
    }
    #[diplomat::opaque]
    pub struct BaadUtilsResponse(pub Option<reqwest::Response>);
    impl BaadUtilsResponse {
        pub fn content_length(&self) -> u64 {
            self.0.as_ref().map_or(0, baad_utils::network::get_content_length)
        }

        pub fn status(&self) -> u16 {
            self.0.as_ref().map_or(0, |response| response.status().as_u16())
        }

        pub fn body(&mut self) -> Result<Box<BaadUtilsBytes>, Box<BaadError>> {
            let response = self
                .0
                .take()
                .ok_or_else(|| baad_shared_ffi::error::error("Response body already consumed"))?;
            baad_shared_ffi::error::blocking(response.bytes())
                .map(|bytes| Box::new(BaadUtilsBytes(bytes.into())))
        }
    }
    #[diplomat::opaque]
    pub struct BaadUtilsLineFormatter(pub baad_utils::formatter::LineFormatter);
    impl BaadUtilsLineFormatter {
        pub fn new() -> Box<Self> { Box::new(Self(baad_utils::formatter::LineFormatter::new())) }

        pub fn set_timestamps(&mut self, enabled: bool) {
            self.0 = self.0.clone().with_timestamps(enabled);
        }

        pub const fn includes_timestamps(&self) -> bool { self.0.includes_timestamps() }

        pub fn write_timestamp(&self, output: &mut DiplomatWrite) -> Result<(), Box<BaadError>> {
            self.0.write_timestamp(output).map_err(baad_shared_ffi::error::error)
        }

        pub fn write_level_prefix(
            &self,
            level: BaadUtilsLogLevel,
            success: bool,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BaadError>> {
            self.0
                .write_level_prefix(output, &level.native(), success)
                .map_err(baad_shared_ffi::error::error)
        }

        pub fn write_simple_message(
            &self,
            level: BaadUtilsLogLevel,
            success: bool,
            message: &str,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BaadError>> {
            self.0
                .write_simple_message(output, &level.native(), success, message)
                .map_err(baad_shared_ffi::error::error)
        }

        pub fn write_line(
            &self,
            level: BaadUtilsLogLevel,
            success: bool,
            message: &str,
            names: &[DiplomatStrSlice],
            values: &[DiplomatStrSlice],
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BaadError>> {
            let fields =
                crate::adapter::fields(names, values).map_err(baad_shared_ffi::error::error)?;
            self.0
                .write_line(output, &level.native(), success, message, &fields)
                .map_err(baad_shared_ffi::error::error)
        }

        pub fn write_line_aligned<'a>(
            &self,
            line: BaadUtilsAlignedLine<'a>,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BaadError>> {
            self.0.write_line_aligned(output, &line.into()).map_err(baad_shared_ffi::error::error)
        }
    }
    impl BaadUtilsLogLevel {
        pub const fn visual_length(self, success: bool) -> usize {
            baad_utils::formatter::styles::level_visual_length(&self.native(), success)
        }

        pub const fn index(self) -> usize {
            baad_utils::formatter::styles::level_to_index(&self.native())
        }

        pub fn style(self) -> Box<BaadUtilsStyle> {
            Box::new(BaadUtilsStyle(baad_utils::formatter::styles::level_style(&self.native())))
        }

        pub fn value_style(self) -> Box<BaadUtilsStyle> {
            Box::new(BaadUtilsStyle(baad_utils::formatter::styles::value_style(&self.native())))
        }
    }
    #[diplomat::opaque]
    pub struct BaadUtilsStyle(pub owo_colors::Style);
    impl BaadUtilsStyle {
        pub fn apply(&self, text: &str, output: &mut DiplomatWrite) -> Result<(), Box<BaadError>> {
            use owo_colors::OwoColorize;
            write!(output, "{}", text.style(self.0)).map_err(baad_shared_ffi::error::error)
        }
    }
    #[diplomat::opaque]
    pub struct BaadUtilsProgressDisplay(pub Option<baad_utils::progress::ProgressDisplay>);
    impl BaadUtilsProgressDisplay {
        pub fn new() -> Box<Self> {
            Box::new(Self(Some(baad_utils::progress::ProgressDisplay::new())))
        }

        pub fn handle_event(
            &mut self,
            event: &BaadSharedProgressEvent
        ) -> Result<(), Box<BaadError>> {
            use baad_utils::progress::ProgressHandler;
            self.0
                .as_mut()
                .ok_or_else(|| baad_shared_ffi::error::error("Progress display already consumed"))?
                .handle_event(event.0.clone());
            Ok(())
        }

        pub fn render(
            &mut self,
            width: usize,
            height: usize,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BaadError>> {
            use baad_utils::progress::ProgressModel;
            let mut text = String::new();
            self.0
                .as_mut()
                .ok_or_else(|| baad_shared_ffi::error::error("Progress display already consumed"))?
                .render(width, height, &mut text);
            output.write_str(&text).map_err(baad_shared_ffi::error::error)
        }
    }
    #[diplomat::opaque]
    pub struct BaadUtilsProgressView(
        pub Option<baad_utils::progress::ProgressView<baad_utils::progress::ProgressDisplay>>
    );
    impl BaadUtilsProgressView {
        pub fn new(
            display: &mut BaadUtilsProgressDisplay,
            interval_ms: u64
        ) -> Result<Box<Self>, Box<BaadError>> {
            let display = display.0.take().ok_or_else(|| {
                baad_shared_ffi::error::error("Progress display already consumed")
            })?;
            Ok(Box::new(Self(Some(baad_utils::progress::ProgressView::new(
                display,
                std::time::Duration::from_millis(interval_ms)
            )))))
        }

        pub fn handle_event(&self, event: &BaadSharedProgressEvent) -> Result<(), Box<BaadError>> {
            use baad_utils::progress::ProgressHandler;
            self.0
                .as_ref()
                .ok_or_else(|| baad_shared_ffi::error::error("Progress view already finished"))?
                .update(|display| display.handle_event(event.0.clone()))
                .map_err(baad_shared_ffi::error::error)
        }

        pub fn message(&self, message: &str) -> Result<(), Box<BaadError>> {
            self.0
                .as_ref()
                .ok_or_else(|| baad_shared_ffi::error::error("Progress view already finished"))?
                .message(message)
                .map_err(baad_shared_ffi::error::error)
        }

        pub fn clear(&self) -> Result<(), Box<BaadError>> {
            self.0
                .as_ref()
                .ok_or_else(|| baad_shared_ffi::error::error("Progress view already finished"))?
                .clear()
                .map_err(baad_shared_ffi::error::error)
        }

        pub fn finish(&mut self) -> Result<Box<BaadUtilsProgressDisplay>, Box<BaadError>> {
            let view = self
                .0
                .take()
                .ok_or_else(|| baad_shared_ffi::error::error("Progress view already finished"))?;
            view.finish()
                .map(|display| Box::new(BaadUtilsProgressDisplay(Some(display))))
                .map_err(baad_shared_ffi::error::error)
        }

        pub fn write(&self, bytes: &[u8]) -> Result<usize, Box<BaadError>> {
            use std::io::Write;
            let mut view = self
                .0
                .as_ref()
                .ok_or_else(|| baad_shared_ffi::error::error("Progress view already finished"))?;
            view.write(bytes).map_err(baad_shared_ffi::error::error)
        }
    }
}

impl ffi::BaadUtilsLogLevel {
    const fn native(self) -> tracing::Level { *self.native_ref() }

    const fn native_ref(self) -> &'static tracing::Level {
        match self {
            Self::Trace => &tracing::Level::TRACE,
            Self::Debug => &tracing::Level::DEBUG,
            Self::Info => &tracing::Level::INFO,
            Self::Warn => &tracing::Level::WARN,
            Self::Error => &tracing::Level::ERROR
        }
    }
}
