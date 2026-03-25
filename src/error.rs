use thiserror::Error;

pub type Result<T> = std::result::Result<T, LetItError>;

#[derive(Debug, Error)]
pub enum LetItError {
    #[error("api error: {0}")]
    Api(String),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
