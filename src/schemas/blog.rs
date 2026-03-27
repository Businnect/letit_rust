use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Default)]
pub struct ListAdminBlogsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl ListAdminBlogsParams {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AdminBlogSummary {
    pub body: String,
    pub category: String,
    pub cover: String,
    pub is_featured: bool,
    pub published_at: String,
    pub slug: String,
    pub summary: String,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AdminBlogListResponse {
    pub list: Vec<AdminBlogSummary>,
    pub total_list: i64,
    pub total_pages: i64,
}

pub type AdminBlogArticle = serde_json::Value;
