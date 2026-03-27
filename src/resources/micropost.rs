use std::sync::Arc;

use reqwest::multipart::{Form, Part};
use reqwest::Method;
use serde_json::json;

use crate::client::ClientInner;
use crate::error::Result;
use crate::schemas::{CreatedWithPublicIdAndLink, FilePayload, MicropostVoteEdited, PostType};

#[derive(Clone)]
pub struct MicropostResource {
    inner: Arc<ClientInner>,
}

impl MicropostResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn create(
        &self,
        request: CreateMicropostRequest,
    ) -> Result<CreatedWithPublicIdAndLink> {
        let mut form = Form::new()
            .text("body", request.body)
            .text("post_type", request.post_type.as_str().to_string())
            .text("allow_comments", request.allow_comments.to_string())
            .text("is_draft", request.is_draft.to_string());

        if let Some(title) = request.title {
            form = form.text("title", title);
        }

        if let Some(file) = request.file {
            let part = file_part(file)?;
            form = form.part("file", part);
        }

        let body = self
            .inner
            .send_text(
                self.inner
                    .request(Method::POST, "/api/v1/client/micropost")
                    .multipart(form),
            )
            .await?;

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn delete(&self, public_id: impl AsRef<str>) -> Result<()> {
        let payload = json!({ "public_id": public_id.as_ref() });

        self.inner
            .send_text(
                self.inner
                    .request(Method::DELETE, "/api/v1/client/micropost")
                    .json(&payload),
            )
            .await?;

        Ok(())
    }

    pub async fn vote(&self, public_id: impl AsRef<str>) -> Result<MicropostVoteEdited> {
        let payload = json!({ "public_id": public_id.as_ref() });

        let body = self
            .inner
            .send_text(
                self.inner
                    .request(Method::PATCH, "/api/v1/client/micropost/vote")
                    .json(&payload),
            )
            .await?;

        Ok(serde_json::from_str(&body)?)
    }
}

#[derive(Debug, Clone)]
pub struct CreateMicropostRequest {
    pub body: String,
    pub title: Option<String>,
    pub post_type: PostType,
    pub allow_comments: bool,
    pub is_draft: bool,
    pub file: Option<FilePayload>,
}

impl CreateMicropostRequest {
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            ..Default::default()
        }
    }
}

impl Default for CreateMicropostRequest {
    fn default() -> Self {
        Self {
            body: String::new(),
            title: None,
            post_type: PostType::Text,
            allow_comments: true,
            is_draft: false,
            file: None,
        }
    }
}

fn file_part(file: FilePayload) -> Result<Part> {
    let mut part = Part::bytes(file.bytes).file_name(file.filename);

    if let Some(mime_type) = file.mime_type {
        part = part.mime_str(&mime_type)?;
    }

    Ok(part)
}
