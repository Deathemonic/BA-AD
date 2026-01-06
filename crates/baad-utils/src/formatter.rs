use crate::utils::{contains_url, format_urls, get_level_visual_length, level_to_index};

use std::borrow::Cow;
use std::fmt;
use std::sync::Arc;

use chrono::{DateTime, Local};
use owo_colors::{OwoColorize, Stream, Style};
use smallvec::SmallVec;
use tracing::{
    Event, Level, Subscriber,
    field::{Field, Visit},
};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields, format::Writer};
use tracing_subscriber::registry::LookupSpan;

const LEVEL_PREFIXES: &[&str] = &["[ERROR]", "[WARN]", "[INFO]", "[DEBUG]", "[TRACE]"];
const SUCCESS_PREFIX: &str = "[SUCCESS]";
const CAUSE_PREFIX: &str = "[CAUSE]";

const TIMESTAMP_STYLE: Style = Style::new().bright_black();
const ERROR_STYLE: Style = Style::new().red().bold();
const WARN_STYLE: Style = Style::new().yellow().bold();
const INFO_STYLE: Style = Style::new().blue().bold();
const DEBUG_STYLE: Style = Style::new().cyan().bold();
const TRACE_STYLE: Style = Style::new().magenta().bold();
const SUCCESS_STYLE: Style = Style::new().green().bold();
const CAUSE_STYLE: Style = Style::new().truecolor(255, 165, 0).bold();

const ERROR_VALUE_STYLE: Style = Style::new().red().italic();
const WARN_VALUE_STYLE: Style = Style::new().yellow().italic();
const INFO_VALUE_STYLE: Style = Style::new().blue().italic();
const DEBUG_VALUE_STYLE: Style = Style::new().cyan().italic();
const TRACE_VALUE_STYLE: Style = Style::new().magenta().italic();
const SUCCESS_VALUE_STYLE: Style = Style::new().green().italic();
const CAUSE_VALUE_STYLE: Style = Style::new().truecolor(255, 165, 0).italic();

#[derive(Clone)]
pub struct ConsoleFormatter {
    config: Arc<FormatterConfig>,
}

#[derive(Debug, Clone)]
struct FormatterConfig {
    include_timestamps: bool,
    include_spans: bool,
}

impl Default for ConsoleFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsoleFormatter {
    pub fn new() -> Self {
        Self {
            config: Arc::new(FormatterConfig {
                include_timestamps: false,
                include_spans: false,
            }),
        }
    }

    #[inline]
    fn get_level_style(level: &Level) -> Style {
        match *level {
            Level::ERROR => ERROR_STYLE,
            Level::WARN => WARN_STYLE,
            Level::INFO => INFO_STYLE,
            Level::DEBUG => DEBUG_STYLE,
            Level::TRACE => TRACE_STYLE,
        }
    }

    pub fn with_timestamps(mut self, include_timestamps: bool) -> Self {
        Arc::make_mut(&mut self.config).include_timestamps = include_timestamps;
        self
    }

    pub fn with_spans(mut self, include_spans: bool) -> Self {
        Arc::make_mut(&mut self.config).include_spans = include_spans;
        self
    }

    fn write_timestamp(&self, writer: &mut Writer<'_>) -> fmt::Result {
        let now: DateTime<Local> = Local::now();
        let timestamp = now.format("%H:%M:%S");

        write!(
            writer,
            "{}",
            timestamp.if_supports_color(Stream::Stdout, |t| t.style(TIMESTAMP_STYLE))
        )
    }

    fn write_level_prefix(
        &self,
        writer: &mut Writer<'_>,
        level: &Level,
        is_success: bool,
    ) -> fmt::Result {
        let visual_length = get_level_visual_length(level, is_success);
        let padding = 9_usize.saturating_sub(visual_length);

        write!(writer, "{:width$}", "", width = padding)?;

        if is_success {
            write!(
                writer,
                "{}",
                SUCCESS_PREFIX.if_supports_color(Stream::Stdout, |t| t.style(SUCCESS_STYLE))
            )
        } else {
            let prefix = LEVEL_PREFIXES[level_to_index(level)];
            let style = Self::get_level_style(level);
            write!(
                writer,
                "{}",
                prefix.if_supports_color(Stream::Stdout, |t| t.style(style))
            )
        }
    }

    fn write_simple_message(
        &self,
        writer: &mut Writer<'_>,
        level: &Level,
        is_success: bool,
        fields: &[(&'static str, Cow<'static, str>)],
    ) -> fmt::Result {
        self.write_level_prefix(writer, level, is_success)?;
        write!(writer, " ")?;

        if let Some((_, message)) = fields.first() {
            write!(writer, "{}", message)?;
        }

        Ok(())
    }

    fn write_cause_line(&self, writer: &mut Writer<'_>, cause_value: &str) -> fmt::Result {
        if self.config.include_timestamps {
            self.write_timestamp(writer)?;
            write!(writer, " ")?;
        }

        let visual_length = 7;
        let padding = 9_usize.saturating_sub(visual_length);

        write!(
            writer,
            "{:width$}{} ",
            "",
            CAUSE_PREFIX.if_supports_color(Stream::Stdout, |t| t.style(CAUSE_STYLE)),
            width = padding
        )?;

        if contains_url(cause_value) {
            let formatted = format_urls(
                cause_value,
                |text| {
                    format!(
                        "{}",
                        text.if_supports_color(Stream::Stdout, |t| t.style(CAUSE_VALUE_STYLE))
                    )
                },
                |url| {
                    format!(
                        "{}",
                        url.if_supports_color(Stream::Stdout, |t| t
                            .style(CAUSE_VALUE_STYLE.underline()))
                    )
                },
            );
            write!(writer, "{}", formatted)?;
        } else {
            write!(writer, "{}", cause_value)?;
        }

        writeln!(writer)
    }
}

impl<S, N> FormatEvent<S, N> for ConsoleFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let level = event.metadata().level();
        let mut visitor = FieldCollector::new();
        event.record(&mut visitor);

        let is_success = level == &Level::INFO && visitor.has_success_field();

        if visitor.is_simple_message() && !self.config.include_timestamps {
            self.write_simple_message(&mut writer, level, is_success, &visitor.fields)?;
            return writeln!(writer);
        }

        if self.config.include_timestamps {
            self.write_timestamp(&mut writer)?;
            write!(writer, " ")?;
        }

        self.write_level_prefix(&mut writer, level, is_success)?;
        write!(writer, " ")?;

        let formatter = FieldFormatter::new(level, is_success);
        formatter.write_fields(&mut writer, &visitor.fields)?;

        writeln!(writer)?;

        if let Some(cause_value) = visitor.get_cause_value() {
            self.write_cause_line(&mut writer, cause_value)?;
        }

        Ok(())
    }
}

struct FieldCollector {
    fields: SmallVec<[(&'static str, Cow<'static, str>); 4]>,
}

impl FieldCollector {
    #[inline]
    fn new() -> Self {
        Self {
            fields: SmallVec::new(),
        }
    }

    #[inline]
    fn has_success_field(&self) -> bool {
        self.fields
            .iter()
            .any(|(name, value)| *name == "success" && value == "true")
    }

    #[inline]
    fn is_simple_message(&self) -> bool {
        self.fields.len() == 1
            && self
                .fields
                .first()
                .map(|(name, _)| *name == "message")
                .unwrap_or(false)
    }

    #[inline]
    fn get_cause_value(&self) -> Option<&str> {
        self.fields
            .iter()
            .find(|(name, _)| *name == "cause")
            .map(|(_, value)| value.as_ref())
    }
}

impl Visit for FieldCollector {
    fn record_i64(&mut self, field: &Field, value: i64) {
        let mut buf = itoa::Buffer::new();
        self.fields
            .push((field.name(), Cow::Owned(buf.format(value).to_owned())));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        let mut buf = itoa::Buffer::new();
        self.fields
            .push((field.name(), Cow::Owned(buf.format(value).to_owned())));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.fields.push((
            field.name(),
            Cow::Borrowed(if value { "true" } else { "false" }),
        ));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields
            .push((field.name(), Cow::Owned(value.to_owned())));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.fields
            .push((field.name(), Cow::Owned(format!("{:?}", value))));
    }
}

struct FieldFormatter<'a> {
    level: &'a Level,
    is_success: bool,
}

impl<'a> FieldFormatter<'a> {
    #[inline]
    fn new(level: &'a Level, is_success: bool) -> Self {
        Self { level, is_success }
    }

    #[inline]
    fn get_value_style(level: &Level) -> Style {
        match *level {
            Level::ERROR => ERROR_VALUE_STYLE,
            Level::WARN => WARN_VALUE_STYLE,
            Level::INFO => INFO_VALUE_STYLE,
            Level::DEBUG => DEBUG_VALUE_STYLE,
            Level::TRACE => TRACE_VALUE_STYLE,
        }
    }

    fn write_fields(
        &self,
        writer: &mut Writer<'_>,
        fields: &[(&'static str, Cow<'static, str>)],
    ) -> fmt::Result {
        let non_message_fields: SmallVec<[&(&'static str, Cow<'static, str>); 4]> = fields
            .iter()
            .filter(|(name, _)| *name != "message" && *name != "success" && *name != "cause")
            .collect();

        if let Some((_, message)) = fields.iter().find(|(name, _)| *name == "message") {
            write!(writer, "{}", message)?;
        }

        for (i, (field_name, value)) in non_message_fields.iter().enumerate() {
            self.write_field(writer, field_name, value, non_message_fields.len(), i == 0)?;
        }

        Ok(())
    }

    fn write_field(
        &self,
        writer: &mut Writer<'_>,
        field_name: &str,
        value: &str,
        field_count: usize,
        is_first: bool,
    ) -> fmt::Result {
        if field_count == 1 {
            write!(writer, ": ")?;
            self.write_colored_value(writer, value)?;
            return Ok(());
        }

        let separator = if is_first { ": " } else { ", " };
        write!(writer, "{}", separator)?;
        self.write_colored_field(writer, field_name, value)?;

        Ok(())
    }

    fn write_colored_value(&self, writer: &mut Writer<'_>, value: &str) -> fmt::Result {
        if self.is_success {
            return write!(
                writer,
                "{}",
                value.if_supports_color(Stream::Stdout, |t| t.style(SUCCESS_VALUE_STYLE))
            );
        }

        if !contains_url(value) {
            let style = Self::get_value_style(self.level);
            write!(
                writer,
                "{}",
                value.if_supports_color(Stream::Stdout, |t| t.style(style))
            )
        } else {
            let formatted = self.format_with_urls(value);
            write!(writer, "{}", formatted)
        }
    }

    fn write_colored_field(
        &self,
        writer: &mut Writer<'_>,
        field_name: &str,
        value: &str,
    ) -> fmt::Result {
        let style = if self.is_success {
            SUCCESS_VALUE_STYLE
        } else {
            Self::get_value_style(self.level)
        };

        write!(
            writer,
            "{}=",
            field_name.if_supports_color(Stream::Stdout, |t| t.style(style))
        )?;
        self.write_colored_value(writer, value)?;
        Ok(())
    }

    fn format_with_urls(&self, value: &str) -> String {
        format_urls(
            value,
            |text| self.format_by_level(text, false),
            |url| self.format_by_level(url, true),
        )
    }

    fn format_by_level(&self, value: &str, is_url: bool) -> String {
        let style = Self::get_value_style(self.level);
        let style = if is_url { style.underline() } else { style };
        format!(
            "{}",
            value.if_supports_color(Stream::Stdout, |t| t.style(style))
        )
    }
}
