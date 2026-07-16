use std::fmt;

use lazy_regex::regex;
use owo_colors::Style;
use tracing::Level;

pub const LEVEL_PREFIXES: &[&str] = &["[ERROR]", "[WARNING]", "[INFO]", "[DEBUG]", "[TRACE]"];
pub const SUCCESS_PREFIX: &str = "[SUCCESS]";
pub const CAUSE_PREFIX: &str = "[CAUSE]";

pub const TIMESTAMP_STYLE: Style = Style::new().bright_black();
pub const ERROR_STYLE: Style = Style::new().red().bold();
pub const WARN_STYLE: Style = Style::new().yellow().bold();
pub const INFO_STYLE: Style = Style::new().blue().bold();
pub const DEBUG_STYLE: Style = Style::new().cyan().bold();
pub const TRACE_STYLE: Style = Style::new().magenta().bold();
pub const SUCCESS_STYLE: Style = Style::new().green().bold();
pub const CAUSE_STYLE: Style = Style::new().truecolor(255, 165, 0).bold();

pub const ERROR_VALUE_STYLE: Style = Style::new().red().italic();
pub const WARN_VALUE_STYLE: Style = Style::new().yellow().italic();
pub const INFO_VALUE_STYLE: Style = Style::new().blue().italic();
pub const DEBUG_VALUE_STYLE: Style = Style::new().cyan().italic();
pub const TRACE_VALUE_STYLE: Style = Style::new().magenta().italic();
pub const SUCCESS_VALUE_STYLE: Style = Style::new().green().italic();
pub const CAUSE_VALUE_STYLE: Style = Style::new().truecolor(255, 165, 0).italic();

#[inline]
pub const fn level_style(level: &Level) -> Style {
    match *level {
        Level::ERROR => ERROR_STYLE,
        Level::WARN => WARN_STYLE,
        Level::INFO => INFO_STYLE,
        Level::DEBUG => DEBUG_STYLE,
        Level::TRACE => TRACE_STYLE
    }
}

#[inline]
pub const fn value_style(level: &Level) -> Style {
    match *level {
        Level::ERROR => ERROR_VALUE_STYLE,
        Level::WARN => WARN_VALUE_STYLE,
        Level::INFO => INFO_VALUE_STYLE,
        Level::DEBUG => DEBUG_VALUE_STYLE,
        Level::TRACE => TRACE_VALUE_STYLE
    }
}

#[inline]
pub const fn level_to_index(level: &Level) -> usize {
    match *level {
        Level::ERROR => 0,
        Level::WARN => 1,
        Level::INFO => 2,
        Level::DEBUG => 3,
        Level::TRACE => 4
    }
}

#[inline]
pub const fn level_visual_length(level: &Level, is_success: bool) -> usize {
    if is_success {
        return SUCCESS_PREFIX.len();
    }
    LEVEL_PREFIXES[level_to_index(level)].len()
}

#[inline]
pub fn contains_url(value: &str) -> bool {
    if !value.contains("://") {
        return false;
    }
    regex!(r"https?://[^\s]+|ftp://[^\s]+").is_match(value)
}

#[inline]
pub fn format_urls<W, F1, F2>(
    content: &str,
    writer: &mut W,
    format_text: F1,
    format_url: F2
) -> fmt::Result
where
    W: fmt::Write,
    F1: Fn(&mut W, &str) -> fmt::Result,
    F2: Fn(&mut W, &str) -> fmt::Result
{
    if !content.contains("://") {
        return format_text(writer, content);
    }
    let url_regex = regex!(r"https?://[^\s]+|ftp://[^\s]+");
    if !url_regex.is_match(content) {
        return format_text(writer, content);
    }
    let mut last_end = 0;
    for mat in url_regex.find_iter(content) {
        if mat.start() > last_end {
            format_text(writer, &content[last_end..mat.start()])?;
        }
        format_url(writer, mat.as_str())?;
        last_end = mat.end();
    }
    if last_end < content.len() {
        format_text(writer, &content[last_end..])?;
    }
    Ok(())
}
