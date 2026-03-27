use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostType {
    Text,
    Media,
}

impl PostType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "TEXT",
            Self::Media => "MEDIA",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MicropostVoteEdited {
    pub user_voted: bool,
}
