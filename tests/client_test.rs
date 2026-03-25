mod common;

use letit::{CreateMicropostRequest, LetItClient};

#[tokio::test]
#[ignore = "hits the production LetIt API"]
async fn invalid_api_token_returns_api_error() {
    let client = LetItClient::new(common::PRODUCTION_BASE_URL, "fake-key");
    let mut request = CreateMicropostRequest::new("Hello");
    request.title = Some("Test".to_string());

    let error = client.micropost.create(request).await.unwrap_err();
    let message = error.to_string();

    assert!(
        message.contains("401") || message.contains("USER-API-TOKEN"),
        "unexpected production error: {message}"
    );
}
