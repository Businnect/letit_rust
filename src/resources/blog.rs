use std::sync::Arc;

use reqwest::Method;

use crate::client::ClientInner;
use crate::error::Result;
use crate::schemas::{AdminBlogArticle, AdminBlogListResponse, ListAdminBlogsParams};

#[derive(Clone)]
pub struct BlogResource {
    inner: Arc<ClientInner>,
}

impl BlogResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn list_admin(&self, params: ListAdminBlogsParams) -> Result<AdminBlogListResponse> {
        let body = self
            .inner
            .send_text(
                self.inner
                    .request(Method::GET, "/api/v1/client/admin/blog/list")
                    .query(&params),
            )
            .await?;

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn get_admin(&self, slug: impl AsRef<str>) -> Result<Option<AdminBlogArticle>> {
        let query = [("slug", slug.as_ref())];
        let body = self
            .inner
            .send_text(
                self.inner
                    .request(Method::GET, "/api/v1/client/admin/blog")
                    .query(&query),
            )
            .await?;

        if body.trim() == "null" {
            return Ok(None);
        }

        Ok(Some(serde_json::from_str(&body)?))
    }
}
