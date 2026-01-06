use std::future::Future;

use eyre::Result;
use lazy_regex::regex;
use tracing::{error, Level};

#[inline]
pub fn contains_url(value: &str) -> bool {
    if !value.contains("://") {
        return false;
    }
    regex!(r"https?://[^\s]+|ftp://[^\s]+").is_match(value)
}

#[inline]
pub fn format_urls<F1, F2>(content: &str, format_text: F1, format_url: F2) -> String
where
    F1: Fn(&str) -> String,
    F2: Fn(&str) -> String,
{
    if !content.contains("://") {
        return format_text(content);
    }

    let url_regex = regex!(r"https?://[^\s]+|ftp://[^\s]+");

    if !url_regex.is_match(content) {
        return format_text(content);
    }

    let mut result = String::new();
    let mut last_end = 0;

    for mat in url_regex.find_iter(content) {
        if mat.start() > last_end {
            let before_url = &content[last_end..mat.start()];
            result.push_str(&format_text(before_url));
        }

        result.push_str(&format_url(mat.as_str()));
        last_end = mat.end();
    }

    if last_end < content.len() {
        let after_url = &content[last_end..];
        result.push_str(&format_text(after_url));
    }

    result
}

#[inline]
pub fn level_to_index(level: &Level) -> usize {
    match *level {
        Level::ERROR => 0,
        Level::WARN => 1,
        Level::INFO => 2,
        Level::DEBUG => 3,
        Level::TRACE => 4,
    }
}

#[inline]
pub fn get_level_visual_length(level: &Level, is_success: bool) -> usize {
    if is_success {
        return 9;
    }
    match *level {
        Level::ERROR | Level::DEBUG | Level::TRACE => 7,
        Level::WARN | Level::INFO => 6,
    }
}

pub fn run<F>(f: F)
where
    F: FnOnce() -> Result<()>,
{
    if let Err(e) = crate::config::init_logging_default() {
        error!("Failed to initialize logging: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = f() {
        error!("{:?}", e);
        std::process::exit(1);
    }
}

pub async fn run_async<F, Fut>(f: F)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<()>>,
{
    if let Err(e) = crate::config::init_logging_default() {
        error!("Failed to initialize logging: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = f().await {
        error!("{:?}", e);
        std::process::exit(1);
    }
}
