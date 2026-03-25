mod common;

use letit::{CreateMicropostRequest, PostType};

#[tokio::test]
#[ignore = "hits the production LetIt API"]
async fn create_and_delete_micropost() {
    let client = common::live_client();
    let body = common::unique_name("The Rust SDK is now live");
    let title = common::unique_name("New Update");
    let mut request = CreateMicropostRequest::new(&body);
    request.title = Some(title);
    request.post_type = PostType::Text;

    let response = client.micropost.create(request).await.unwrap();
    assert!(!response.public_id.is_empty());
    assert!(!response.link.trim().is_empty());

    client.micropost.delete(&response.public_id).await.unwrap();
}
