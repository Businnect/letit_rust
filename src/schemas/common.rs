use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct FilePayload {
    pub filename: String,
    pub bytes: Vec<u8>,
    pub mime_type: Option<String>,
}

impl FilePayload {
    pub fn new(filename: impl Into<String>, bytes: Vec<u8>) -> Self {
        Self {
            filename: filename.into(),
            bytes,
            mime_type: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreatedWithPublicIdAndLink {
    #[serde(rename = "public_id")]
    pub public_id: String,
    pub link: String,
}
