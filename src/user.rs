use crate::error::auth_error::AuthError;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, FromRow, Validate)]
pub struct UserRequest {
    pub user_request_type: UserRequestType,
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: Option<String>,
    #[validate(email(message = "Invalid email address"))]
    pub email: Option<String>,
    #[validate(length(
        min = 10,
        max = 15,
        message = "Phone number must be between 10 and 15 characters"
    ))]
    pub phone: Option<String>,
    pub wechat: Option<String>,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub app_name: String,
}

impl UserRequest {
    pub fn validate(&self) -> Result<(), AuthError> {
        if self.username.is_none() && self.email.is_none() && self.phone.is_none() {
            return Err(AuthError::ValidationError(
                "至少需要提供 username/email/phone 之一".into(),
            ));
        }
        Ok(())
    }
    pub fn validate_password(password: &str) -> Result<(), AuthError> {
        if password.len() < 8 {
            return Err(AuthError::PasswordError("密码至少8位".into()));
        }
        // 添加复杂度规则
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum UserRequestType {
    Username, // 用户名登录
    Email,    // 邮箱登录
    Phone,    // 手机号登录
    WeChat,
}

#[derive(Debug, Deserialize, Serialize, FromRow, Validate)]
pub struct LoginRequest {
    pub login_type: UserRequestType, // 反序列化为枚举类型
    pub identity: String,
}
