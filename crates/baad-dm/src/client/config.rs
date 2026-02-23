use bon::Builder;
use reqwest_middleware::reqwest::Proxy;
use reqwest_middleware::reqwest::header::HeaderMap;

#[derive(Debug, Clone, Builder)]
pub struct HttpClientConfig {
    #[builder(default = 3)]
    pub retries: u32,

    pub proxy: Option<Proxy>,

    pub headers: Option<HeaderMap>,

    #[builder(default = 10)]
    pub pool_max_idle: usize,

    #[builder(default = true)]
    pub tcp_nodelay: bool
}
