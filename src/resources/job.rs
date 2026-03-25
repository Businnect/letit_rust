use std::sync::Arc;

use reqwest::multipart::{Form, Part};
use reqwest::Method;
use serde_json::json;

use crate::client::ClientInner;
use crate::error::Result;
use crate::schemas::{FilePayload, JobLocation, UserJobCreatedByUser};

#[derive(Clone)]
pub struct JobResource {
    inner: Arc<ClientInner>,
}

impl JobResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn create_with_company(
        &self,
        request: CreateJobWithCompanyRequest,
    ) -> Result<UserJobCreatedByUser> {
        let mut form = Form::new()
            .text("company_name", request.company_name)
            .text("company_description", request.company_description)
            .text("company_website", request.company_website)
            .text("job_title", request.job_title)
            .text("job_description", request.job_description)
            .text("job_how_to_apply", request.job_how_to_apply)
            .text(
                "job_pay_in_cryptocurrency",
                request.job_pay_in_cryptocurrency.to_string(),
            )
            .text("job_location", request.job_location.as_str().to_string());

        if let Some(company_logo) = request.company_logo {
            let part = file_part(company_logo)?;
            form = form.part("company_logo", part);
        }

        let body = self
            .inner
            .send_text(
                self.inner
                    .request(Method::POST, "/api/v1/client/job")
                    .multipart(form),
            )
            .await?;

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn delete(&self, slug: impl AsRef<str>) -> Result<()> {
        let payload = json!({ "slug": slug.as_ref() });

        self.inner
            .send_text(
                self.inner
                    .request(Method::DELETE, "/api/v1/client/job")
                    .json(&payload),
            )
            .await?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CreateJobWithCompanyRequest {
    pub company_name: String,
    pub company_description: String,
    pub company_website: String,
    pub job_title: String,
    pub job_description: String,
    pub job_how_to_apply: String,
    pub job_pay_in_cryptocurrency: bool,
    pub company_logo: Option<FilePayload>,
    pub job_location: JobLocation,
}

impl CreateJobWithCompanyRequest {
    pub fn new(
        company_name: impl Into<String>,
        company_description: impl Into<String>,
        company_website: impl Into<String>,
        job_title: impl Into<String>,
        job_description: impl Into<String>,
        job_how_to_apply: impl Into<String>,
    ) -> Self {
        Self {
            company_name: company_name.into(),
            company_description: company_description.into(),
            company_website: company_website.into(),
            job_title: job_title.into(),
            job_description: job_description.into(),
            job_how_to_apply: job_how_to_apply.into(),
            job_pay_in_cryptocurrency: false,
            company_logo: None,
            job_location: JobLocation::Remote,
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
