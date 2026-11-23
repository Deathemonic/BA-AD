use std::sync::Arc;

use bytes::Bytes;
use reqwest::{Client, Proxy};
use serde::de::DeserializeOwned;
use serde_json::{Error as JsonError, from_str};

use crate::runtime::CallbackScope;

struct ThreadCallback<F>(Option<F>);

impl<F> Drop for ThreadCallback<F> {
    fn drop(&mut self) {
        let _scope = CallbackScope::enter();
        drop(self.0.take());
    }
}

unsafe impl<F> Send for ThreadCallback<F> {}

unsafe impl<F> Sync for ThreadCallback<F> {}

impl<F: Fn(baad_shared::ProgressEvent) + 'static> baad_shared::ProgressObserver
    for ThreadCallback<F>
{
    fn on_event(&self, event: baad_shared::ProgressEvent) {
        let _scope = CallbackScope::enter();
        if let Some(callback) = &self.0 {
            callback(event);
        }
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn register(callback: impl Fn(baad_shared::ProgressEvent) + 'static) {
    baad_shared::set_observer(Arc::new(ThreadCallback(Some(callback))));
}

pub fn init_client(proxy: &str, user_agent: &str, no_proxy: bool) -> Result<(), String> {
    let mut builder = Client::builder();
    if no_proxy {
        builder = builder.no_proxy();
    }
    if !proxy.is_empty() {
        builder = builder.proxy(Proxy::all(proxy).map_err(|error| error.to_string())?);
    }
    if !user_agent.is_empty() {
        builder = builder.user_agent(user_agent);
    }
    let client = builder.build().map_err(|error| error.to_string())?;
    baad_shared::client::try_set_client(client)
        .map_err(|_| "HTTP client already initialized".into())
}

pub enum ByteBuffer {
    Owned(Vec<u8>),
    Shared(Bytes)
}

impl From<Vec<u8>> for ByteBuffer {
    fn from(value: Vec<u8>) -> Self { Self::Owned(value) }
}

impl From<Bytes> for ByteBuffer {
    fn from(value: Bytes) -> Self { Self::Shared(value) }
}

impl AsRef<[u8]> for ByteBuffer {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Owned(value) => value,
            Self::Shared(value) => value
        }
    }
}

pub fn parse_json<T: DeserializeOwned>(json: &str) -> Result<T, JsonError> { from_str(json) }
