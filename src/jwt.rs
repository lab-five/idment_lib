use super::{error::AuthError, models::Claims};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use pkcs8::LineEnding;
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey}; // 必须显式导入 Trait

pub struct JwtManager {
    config: crate::models::AuthConfig,
}

impl JwtManager {
    pub fn new(config: crate::models::AuthConfig) -> Self {
        Self { config }
    }

    // 生成Access Token（注册中心使用）
    pub fn generate_access_token(
        &self,
        private_key: &RsaPrivateKey,
        claims: Claims,
    ) -> Result<String, AuthError> {
        let header = Header::new(Algorithm::RS256);

        // 使用 pkcs8 的 PEM 编码方法
        let private_pem = private_key
            .to_pkcs8_pem(LineEnding::CRLF)
            .map_err(|e| AuthError::GenerationError(e.to_string()))?
            .to_string();

        let encoding_key = EncodingKey::from_rsa_pem(private_pem.as_bytes())
            .map_err(|e| AuthError::GenerationError(e.to_string()))?;

        encode(&header, &claims, &encoding_key)
            .map_err(|e| AuthError::GenerationError(e.to_string()))
    }

    // 验证Token（子系统使用）
    pub fn validate_token(
        &self,
        public_key: &RsaPublicKey,
        token: &str,
    ) -> Result<Claims, AuthError> {
        // 使用 pkcs8 的公钥编码方法
        let public_pem = public_key
            .to_public_key_pem(LineEnding::CRLF)
            .map_err(|e| AuthError::ValidationError(e.to_string()))?;

        let decoding_key = DecodingKey::from_rsa_pem(public_pem.as_bytes())
            .map_err(|e| AuthError::ValidationError(e.to_string()))?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[self.config.issuer.clone()]);
        validation.set_audience(&[self.config.audience.clone()]);

        let token_data = decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|e| AuthError::ValidationError(e.to_string()))?;

        Ok(token_data.claims)
    }
}
