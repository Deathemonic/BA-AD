use std::fmt;

use better_default::Default;
use chrono::{DateTime, Local};
use owo_colors::{OwoColorize, Stream};
use tracing::Level;

use crate::formatter::styles::{
    CAUSE_PREFIX,
    CAUSE_STYLE,
    CAUSE_VALUE_STYLE,
    LEVEL_PREFIXES,
    SUCCESS_PREFIX,
    SUCCESS_STYLE,
    SUCCESS_VALUE_STYLE,
    TIMESTAMP_STYLE,
    contains_url,
    format_urls,
    level_style,
    level_to_index,
    level_visual_length,
    value_style
};

#[derive(Clone, Default)]
#[default(include_timestamps: true)]
pub struct LineFormatter {
    include_timestamps: bool
}

impl LineFormatter {
    pub fn new() -> Self { Self::default() }

    pub fn with_timestamps(mut self, include_timestamps: bool) -> Self {
        self.include_timestamps = include_timestamps;
        self
    }

    pub fn write_timestamp(&self, writer: &mut impl fmt::Write) -> fmt::Result {
        let now: DateTime<Local> = Local::now();
        let timestamp = now.format("%H:%M:%S");
        write!(
            writer,
            "{}",
            timestamp.if_supports_color(Stream::Stdout, |t| t.style(TIMESTAMP_STYLE))
        )
    }

    pub fn write_level_prefix(
        &self,
        writer: &mut impl fmt::Write,
        level: &Level,
        is_success: bool
    ) -> fmt::Result {
        let visual_length = level_visual_length(level, is_success);
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
            let style = level_style(level);
            write!(writer, "{}", prefix.if_supports_color(Stream::Stdout, |t| t.style(style)))
        }
    }

    pub fn write_simple_message(
        &self,
        writer: &mut impl fmt::Write,
        level: &Level,
        is_success: bool,
        message: &str
    ) -> fmt::Result {
        self.write_level_prefix(writer, level, is_success)?;
        write!(writer, " {}", message)
    }

    pub fn write_line(
        &self,
        writer: &mut impl fmt::Write,
        level: &Level,
        is_success: bool,
        message: &str,
        fields: &[(&str, &str)]
    ) -> fmt::Result {
        if self.include_timestamps {
            self.write_timestamp(writer)?;
            write!(writer, " ")?;
        }

        self.write_level_prefix(writer, level, is_success)?;
        write!(writer, " ")?;

        write!(writer, "{}", message)?;

        let non_meta_fields: Vec<_> =
            fields.iter().filter(|(name, _)| *name != "success" && *name != "cause").collect();

        for (i, (field_name, value)) in non_meta_fields.iter().enumerate() {
            if non_meta_fields.len() == 1 {
                write!(writer, ": ")?;
                write_colored_value(writer, level, is_success, value)?;
            } else {
                let separator = if i == 0 { ": " } else { ", " };
                write!(writer, "{}", separator)?;
                write_colored_field(writer, level, is_success, field_name, value)?;
            }
        }

        writeln!(writer)?;

        if let Some((_, cause_value)) = fields.iter().find(|(name, _)| *name == "cause") {
            self.write_cause_line(writer, cause_value)?;
        }

        Ok(())
    }

    fn write_cause_line(&self, writer: &mut impl fmt::Write, cause_value: &str) -> fmt::Result {
        if self.include_timestamps {
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
                }
            );
            write!(writer, "{}", formatted)?;
        } else {
            write!(writer, "{}", cause_value)?;
        }

        writeln!(writer)
    }

    pub fn includes_timestamps(&self) -> bool { self.include_timestamps }
}

fn write_colored_value(
    writer: &mut impl fmt::Write,
    level: &Level,
    is_success: bool,
    value: &str
) -> fmt::Result {
    if is_success {
        return write!(
            writer,
            "{}",
            value.if_supports_color(Stream::Stdout, |t| t.style(SUCCESS_VALUE_STYLE))
        );
    }

    if !contains_url(value) {
        let style = value_style(level);
        write!(writer, "{}", value.if_supports_color(Stream::Stdout, |t| t.style(style)))
    } else {
        let style = value_style(level);
        let formatted = format_urls(
            value,
            |text| format!("{}", text.if_supports_color(Stream::Stdout, |t| t.style(style))),
            |url| {
                format!("{}", url.if_supports_color(Stream::Stdout, |t| t.style(style.underline())))
            }
        );
        write!(writer, "{}", formatted)
    }
}

fn write_colored_field(
    writer: &mut impl fmt::Write,
    level: &Level,
    is_success: bool,
    field_name: &str,
    value: &str
) -> fmt::Result {
    let style = if is_success { SUCCESS_VALUE_STYLE } else { value_style(level) };

    write!(writer, "{}=", field_name.if_supports_color(Stream::Stdout, |t| t.style(style)))?;
    write_colored_value(writer, level, is_success, value)
}
