use reqwest_middleware::reqwest::Proxy;
use reqwest_middleware::reqwest::header::HeaderMap;

#[derive(Debug, Clone, Default)]
pub struct HttpClientConfig {
    pub retries: u32,
    pub proxy: Option<Proxy>,
    pub headers: Option<HeaderMap>,
    pub pool_max_idle: usize,
    pub tcp_nodelay: bool
}

impl HttpClientConfig {
    pub fn new() -> Self {
        Self {
            retries: 3,
            proxy: None,
            headers: None,
            pool_max_idle: 10,
            tcp_nodelay: true
        }
    }

    pub fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    pub fn with_proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }

    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }
}
