use reqwest::header::{CONTENT_ENCODING, CONTENT_LENGTH, CONTENT_RANGE};
use reqwest::{Response, StatusCode};

use crate::error::Error;

pub fn validate_range_response(
    response: &Response,
    start: u64,
    end: Option<u64>,
    total: Option<u64>
) -> Result<u64, Error> {
    if response.status() != StatusCode::PARTIAL_CONTENT {
        return Err(Error::RangeNotSupported);
    }

    validate_identity(response)?;

    let invalid = || Error::InvalidRangeResponse;
    let headers = response.headers();

    if headers.get_all(CONTENT_RANGE).iter().count() != 1 {
        return Err(invalid());
    }

    let value = headers.get(CONTENT_RANGE).and_then(|value| value.to_str().ok());
    let (range, size) = value
        .and_then(|value| value.strip_prefix("bytes "))
        .and_then(|value| value.split_once('/'))
        .ok_or_else(invalid)?;
    let (first, last) = range.split_once('-').ok_or_else(invalid)?;
    let first = parse_decimal(first).ok_or_else(invalid)?;
    let last = parse_decimal(last).ok_or_else(invalid)?;
    let size = parse_decimal(size).ok_or_else(invalid)?;

    if first != start
        || last < first
        || last >= size
        || end.is_some_and(|end| last != end)
        || end.is_none() && last != size - 1
        || total.is_some_and(|total| total != size)
    {
        return Err(invalid());
    }

    let length =
        last.checked_sub(first).and_then(|length| length.checked_add(1)).ok_or_else(invalid)?;

    if let Some(value) = headers.get(CONTENT_LENGTH)
        && value.to_str().ok().and_then(parse_decimal) != Some(length)
    {
        return Err(invalid());
    }

    Ok(length)
}

pub fn validate_identity(response: &Response) -> Result<(), Error> {
    if response
        .headers()
        .get_all(CONTENT_ENCODING)
        .iter()
        .any(|value| value.as_bytes() != b"identity")
    {
        return Err(Error::EncodedRangeResponse);
    }

    Ok(())
}

fn parse_decimal(value: &str) -> Option<u64> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }

    value.parse().ok()
}
