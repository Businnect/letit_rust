use letit_rust::{CreateMicropostRequest, LetItClient};
use serde_json::json;
use wiremock::matchers::{body_string_contains, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn invalid_api_token_returns_api_error() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/client/micropost"))
        .and(header("USER-API-TOKEN", "fake-key"))
        .and(body_string_contains("Hello"))
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_json(json!({ "detail": "USER-API-TOKEN header is not valid" })),
        )
        .mount(&server)
        .await;

    let client = LetItClient::new(server.uri(), "fake-key");
    let mut request = CreateMicropostRequest::new("Hello");
    request.title = Some("Test".to_string());

    let error = client.micropost.create(request).await.unwrap_err();

    assert!(error
        .to_string()
        .contains("USER-API-TOKEN header is not valid"));
}
