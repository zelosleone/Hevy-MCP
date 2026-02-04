use thiserror::Error;

#[derive(Error, Debug)]
pub enum HevyError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    #[error("Unauthorized: Invalid or missing API key")]
    Unauthorized,

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Rate limited: Please wait before making more requests")]
    RateLimited,

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Serialization error: {message}. Body: {body}")]
    SerializationWithBody { message: String, body: String },
}

pub type Result<T> = std::result::Result<T, HevyError>;
