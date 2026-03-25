use letit_rust::{CreateMicropostRequest, LetItClient, PostType};
use serde_json::json;
use wiremock::matchers::{body_json, body_string_contains, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn create_and_delete_micropost() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/client/micropost"))
        .and(header("USER-API-TOKEN", "test-token"))
        .and(body_string_contains("The Rust SDK is now live!"))
        .and(body_string_contains("TEXT"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "public_id": "rust-post-1",
            "link": "https://letit.com/p/rust-post-1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/api/v1/client/micropost"))
        .and(header("USER-API-TOKEN", "test-token"))
        .and(body_json(json!({ "public_id": "rust-post-1" })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let client = LetItClient::new(server.uri(), "test-token");
    let mut request = CreateMicropostRequest::new("The Rust SDK is now live!");
    request.title = Some("New Update".to_string());
    request.post_type = PostType::Text;

    let response = client.micropost.create(request).await.unwrap();
    assert_eq!(response.public_id, "rust-post-1");
    assert_eq!(response.link, "https://letit.com/p/rust-post-1");

    client.micropost.delete("rust-post-1").await.unwrap();
}
