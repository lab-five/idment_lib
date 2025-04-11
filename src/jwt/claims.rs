use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use serde::{Deserialize, Serialize};

use super::utils::decode_claims_unchecked;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,        // 用户ID
    pub exp: usize,         // 令牌的过期时间 秒
    pub iat: usize,         // 令牌的签发时间
    pub iss: String,        // 令牌的签发者 通常是服务端标识或域名
    pub aud: String,        // 令牌的目标接收者 可以是服务名、客户端ID
    pub token_type: String, // 令牌类型
    pub token_version: i32, // 新增字段
}

// 控制令牌生命周期：exp 定义有效期，iat 用于计算令牌年龄。
// 服务端需验证 exp > 当前时间，否则拒绝请求
// iss 确保令牌来自可信的签发者。
// aud 确保令牌只能用于特定服务（如 aud: "api.my-app.com" 的令牌不能用于 aud: "admin.my-app.com" 的接口）。
// token_type 区分令牌用途：例如 access_token（短期有效）和 refresh_token（长期有效）

// {
//     "token_type": "Bearer",
//     "access_token": "xxxx",
//     "refresh_token": "yyyy"
// }

// token_version 安全增强​：当用户敏感操作（如修改密码、注销设备）时，递增此版本号，使旧版本令牌失效
// if user_db_version ！= token_version {
//     // 令牌已失效，要求重新登录
// }

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub token_version: i32, // 新增字段
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: usize,
}

pub struct AuthClaims(pub TokenClaims); // 你自定义的结构体，里面存解析后的 token claimss

impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| (StatusCode::UNAUTHORIZED, "缺少 Authorization Header".into()))?;

        let token = bearer.token();
        let claims = decode_claims_unchecked(token, "access", true)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "无效的 token".into()))?;

        Ok(AuthClaims(claims))
    }
}
