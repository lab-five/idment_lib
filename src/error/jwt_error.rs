use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Expired token")]
    ExpiredToken,
    #[error("Token creation failed")]
    TokenCreationFailed,
    #[error("Token version mismatch")]
    TokenVersionMismatch,
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum LibError {
   
}
