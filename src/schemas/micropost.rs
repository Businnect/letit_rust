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
