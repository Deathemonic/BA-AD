use std::borrow::Cow;
use std::fmt;

use better_default::Default;
use chrono::{DateTime, Local};
use owo_colors::{OwoColorize, Stream, Style};
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
    format_urls,
    level_style,
    level_to_index,
    level_visual_length,
    value_style
};

pub struct AlignedLine<'a> {
    pub level: &'a Level,
    pub is_success: bool,
    pub message: &'a str,
    pub value: &'a str,
    pub right: &'a str,
    pub width: usize
}

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
        write!(writer, " {message}")
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

        write!(writer, "{message}")?;

        let non_meta_fields: Vec<_> =
            fields.iter().filter(|(name, _)| *name != "success" && *name != "cause").collect();

        for (i, (field_name, value)) in non_meta_fields.iter().enumerate() {
            if non_meta_fields.len() == 1 {
                write!(writer, ": ")?;
                write_colored_value(writer, level, is_success, value)?;
            } else {
                let separator = if i == 0 { ": " } else { ", " };
                write!(writer, "{separator}")?;
                write_colored_field(writer, level, is_success, field_name, value)?;
            }
        }

        writeln!(writer)?;

        if let Some((_, cause_value)) = fields.iter().find(|(name, _)| *name == "cause") {
            self.write_cause_line(writer, cause_value)?;
        }

        Ok(())
    }

    pub fn write_line_aligned(
        &self,
        writer: &mut impl fmt::Write,
        line: &AlignedLine<'_>
    ) -> fmt::Result {
        const LEVEL_COLUMNS: usize = 10;
        const TIMESTAMP_COLUMNS: usize = 9;
        const MIN_GAP: usize = 2;
        const MIN_VALUE_COLUMNS: usize = 8;

        if self.include_timestamps {
            self.write_timestamp(writer)?;
            write!(writer, " ")?;
        }

        self.write_level_prefix(writer, line.level, line.is_success)?;
        write!(writer, " {}: ", line.message)?;

        let timestamp_columns = if self.include_timestamps { TIMESTAMP_COLUMNS } else { 0 };
        let left_columns = timestamp_columns + LEVEL_COLUMNS + line.message.chars().count() + 2;
        let right_columns = line.right.chars().count();

        let reserved = if right_columns > 0 { right_columns + MIN_GAP } else { 0 };
        let value_budget =
            line.width.saturating_sub(left_columns + reserved).max(MIN_VALUE_COLUMNS);

        let truncated = truncate_middle(line.value, value_budget);
        write_colored_value(writer, line.level, line.is_success, &truncated)?;

        if right_columns > 0 {
            let used = left_columns + truncated.chars().count() + right_columns;
            let gap = line.width.saturating_sub(used).max(MIN_GAP);
            write!(
                writer,
                "{:gap$}{}",
                "",
                line.right.if_supports_color(Stream::Stdout, |t| t.style(TIMESTAMP_STYLE))
            )?;
        }

        writeln!(writer)
    }

    fn write_cause_line(&self, writer: &mut impl fmt::Write, cause_value: &str) -> fmt::Result {
        if self.include_timestamps {
            self.write_timestamp(writer)?;
            write!(writer, " ")?;
        }

        let visual_length = CAUSE_PREFIX.len();
        let padding = 9_usize.saturating_sub(visual_length);

        write!(
            writer,
            "{:width$}{} ",
            "",
            CAUSE_PREFIX.if_supports_color(Stream::Stdout, |t| t.style(CAUSE_STYLE)),
            width = padding
        )?;

        write_styled_value(writer, cause_value, CAUSE_VALUE_STYLE)?;

        writeln!(writer)
    }

    pub const fn includes_timestamps(&self) -> bool { self.include_timestamps }
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

    let style = value_style(level);
    write_styled_value(writer, value, style)
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

fn truncate_middle(value: &str, max_chars: usize) -> Cow<'_, str> {
    let count = value.chars().count();
    if count <= max_chars {
        return Cow::Borrowed(value);
    }

    let keep = max_chars.saturating_sub(1);
    let front = keep.div_ceil(2);
    let back = keep / 2;

    let mut truncated = String::with_capacity(max_chars * 4);
    truncated.extend(value.chars().take(front));
    truncated.push('…');
    truncated.extend(value.chars().skip(count - back));

    Cow::Owned(truncated)
}

fn write_styled_value(writer: &mut impl fmt::Write, value: &str, style: Style) -> fmt::Result {
    format_urls(
        value,
        writer,
        |w, text| write!(w, "{}", text.if_supports_color(Stream::Stdout, |t| t.style(style))),
        |w, url| {
            write!(w, "{}", url.if_supports_color(Stream::Stdout, |t| t.style(style.underline())))
        }
    )
}
