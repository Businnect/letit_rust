use letit_rust::{CreateJobWithCompanyRequest, FilePayload, JobLocation, LetItClient};
use serde_json::json;
use wiremock::matchers::{body_json, body_string_contains, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn create_and_delete_job() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/client/job"))
        .and(header("USER-API-TOKEN", "test-token"))
        .and(body_string_contains("Acme Corp"))
        .and(body_string_contains("REMOTE"))
        .and(body_string_contains("logo.png"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "slug": "acme-corp-rust" })))
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/client/job"))
        .and(header("USER-API-TOKEN", "test-token"))
        .and(body_json(json!({ "slug": "acme-corp-rust" })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let client = LetItClient::new(server.uri(), "test-token");
    let mut request = CreateJobWithCompanyRequest::new(
        "Acme Corp",
        "Remote-first company.",
        "https://acme.example",
        "Senior Rust Developer",
        "Build SDKs and APIs.",
        "https://acme.example/apply",
    );
    request.job_location = JobLocation::Remote;
    request.company_logo = Some(FilePayload {
        filename: "logo.png".to_string(),
        bytes: b"fake-logo".to_vec(),
        mime_type: Some("image/png".to_string()),
    });

    let response = client.job.create_with_company(request).await.unwrap();
    assert_eq!(response.slug, "acme-corp-rust");

    client.job.delete("acme-corp-rust").await.unwrap();
}
