use std::sync::Arc;

use reqwest::{Client, Method, RequestBuilder};
use serde_json::Value;

use crate::error::{LetItError, Result};
use crate::resources::{JobResource, MicropostResource};

#[derive(Clone)]
pub struct LetItClient {
    inner: Arc<ClientInner>,
    pub job: JobResource,
    pub micropost: MicropostResource,
}

impl LetItClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        let inner = Arc::new(ClientInner {
            base_url: sanitize_base_url(base_url.into()),
            api_key: api_key.into(),
            http_client: Client::new(),
        });

        Self {
            job: JobResource::new(inner.clone()),
            micropost: MicropostResource::new(inner.clone()),
            inner,
        }
    }

    pub fn with_http_client(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        http_client: Client,
    ) -> Self {
        let inner = Arc::new(ClientInner {
            base_url: sanitize_base_url(base_url.into()),
            api_key: api_key.into(),
            http_client,
        });

        Self {
            job: JobResource::new(inner.clone()),
            micropost: MicropostResource::new(inner.clone()),
            inner,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    pub fn api_key(&self) -> &str {
        &self.inner.api_key
    }
}

pub(crate) struct ClientInner {
    pub(crate) base_url: String,
    pub(crate) api_key: String,
    pub(crate) http_client: Client,
}

impl ClientInner {
    pub(crate) fn request(&self, method: Method, path: &str) -> RequestBuilder {
        self.http_client
            .request(method, format!("{}{}", self.base_url, path))
            .header("USER-API-TOKEN", &self.api_key)
            .header(reqwest::header::USER_AGENT, "LetIt-Rust-SDK/1.0")
    }

    pub(crate) async fn send_text(&self, request: RequestBuilder) -> Result<String> {
        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;

        if status.is_client_error() || status.is_server_error() {
            if let Ok(payload) = serde_json::from_str::<Value>(&body) {
                if let Some(detail) = payload.get("detail").and_then(Value::as_str) {
                    return Err(LetItError::Api(detail.to_string()));
                }
            }

            return Err(LetItError::Api(format!("status {}", status.as_u16())));
        }

        Ok(body)
    }
}

fn sanitize_base_url(base_url: String) -> String {
    base_url.trim_end_matches('/').to_string()
}
