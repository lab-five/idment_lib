use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::{
    auth_error::AuthError, config_error::ConfigError, database_error::DatabaseError,
    jwt_error::JwtError, server_error::ServereError,
};

#[derive(Debug, thiserror::Error)]
pub enum IdmentError {
    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Jwt(#[from] JwtError),

    #[error(transparent)]
    Server(#[from] ServereError),

    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error("内部服务器错误")]
    InternalServerError,
}

impl IntoResponse for IdmentError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::Auth(AuthError::InvalidCredentials) => StatusCode::UNAUTHORIZED,
            Self::Auth(AuthError::UserNotFound) => StatusCode::NOT_FOUND,
            Self::Jwt(JwtError::ExpiredToken) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}

// 无需手动实现 From trait
// 因为使用了 #[from] 属性，自动生成以下实现：
// impl From<AuthError> for  IdmentError  {
//     fn from(e: AuthError) -> Self {
//         AllError::Auth(e)
//     }
// }

// 同理适用于 JwtError
// // 模块独立定义
// #[derive(Error)]
// enum AuthError { /* ... */ }

// #[derive(Error)]
// enum JwtError { /* ... */ }

// // 统一入口
// #[derive(Error)]
// enum AllError {
//     #[error(transparent)]
//     Auth(#[from] AuthError),
//     #[error(transparent)]
//     Jwt(#[from] JwtError)
// }
// error.rs
