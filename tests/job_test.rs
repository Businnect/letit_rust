mod common;

use letit::{CreateJobWithCompanyRequest, FilePayload, JobLocation};

#[tokio::test]
#[ignore = "hits the production LetIt API"]
async fn create_and_delete_job() {
    let client = common::live_client();
    let company_name = common::unique_name("letit-rust-acme");
    let (logo_filename, logo_bytes, logo_mime_type) = common::repository_logo();
    let mut request = CreateJobWithCompanyRequest::new(
        &company_name,
        "Remote-first company.",
        "https://acme.example",
        "Senior Rust Developer",
        "Build SDKs and APIs.",
        "https://acme.example/apply",
    );
    request.job_location = JobLocation::Remote;
    request.company_logo = Some(FilePayload {
        filename: logo_filename,
        bytes: logo_bytes,
        mime_type: Some(logo_mime_type),
    });

    let response = client.job.create_with_company(request).await.unwrap();
    assert!(!response.slug.is_empty());

    client.job.delete(&response.slug).await.unwrap();
}
