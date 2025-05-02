use super::jwt_error::JwtError;
use sqlx::Error as SqlxError;
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error("User not found")]
    UserNotFound,
    #[error("Password Error")]
    PasswordError(String),
    #[error("Token error")]
    TokenError(String),
    #[error("Database error {source}")]
    DatabaseError {
        #[source]
        source: SqlxError,
    },
    #[error("验证失败")]
    ValidationError(String),
    #[error("Hash password error {0}")]
    HashPasswordError(String),
    #[error("Verify password error {0}")]
    VerifyPasswordError(String),
}

impl From<JwtError> for AuthError {
    fn from(err: JwtError) -> Self {
        AuthError::TokenError(err.to_string())
    }
}
