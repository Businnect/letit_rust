mod common;

use letit::ListAdminBlogsParams;

#[tokio::test]
#[ignore = "hits the production LetIt API"]
async fn list_admin_blogs() {
    let client = common::live_client();
    let params = ListAdminBlogsParams {
        limit: Some(5),
        ..Default::default()
    };

    let response = client.blog.list_admin(params).await.unwrap();

    assert!(response.total_list >= 0);
    assert!(response.total_pages >= 0);
}

#[tokio::test]
#[ignore = "hits the production LetIt API"]
async fn get_admin_blog_by_slug() {
    let client = common::live_client();
    let list = client
        .blog
        .list_admin(ListAdminBlogsParams {
            limit: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();

    if let Some(blog) = list.list.first() {
        let response = client.blog.get_admin(&blog.slug).await.unwrap();
        assert!(response.is_some());
    } else {
        let missing_slug = common::unique_name("letit-rust-missing-blog");
        let response = client.blog.get_admin(missing_slug).await.unwrap();
        assert!(response.is_none());
    }
}
