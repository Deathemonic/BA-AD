mod config;
mod http;

pub use config::HttpClientConfig;
pub use http::{
    create_http_client,
    create_range_header,
    get_content_length,
    parse_accept_ranges,
    parse_content_length,
    resolve_url
};
