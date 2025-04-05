use super::{error::AuthError, models::Claims};
use async_trait::async_trait;

/// 刷新令牌存储抽象
#[async_trait]
pub trait TokenStorage {
    /// 存储刷新令牌
    async fn store_refresh_token(&self, user_id: &str, token: &str) -> Result<(), AuthError>;

    /// 验证并获取关联用户ID
    async fn validate_refresh_token(&self, token: &str) -> Result<String, AuthError>;
}

/// 用户认证信息提供抽象
#[async_trait]
pub trait UserProvider {
    /// 用户认证并返回ID
    async fn authenticate(&self, username: &str, password: &str) -> Result<String, AuthError>;

    /// 获取用户声明信息
    async fn get_user_claims(&self, user_id: &str) -> Result<Claims, AuthError>;
}
