use once_cell::sync::Lazy;
use std::env;

pub static JWT_CONFIG: Lazy<JwtConfig> = Lazy::new(|| JwtConfig::from_env());

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub access_token_secret: String,
    pub refresh_token_secret: String,
    pub access_token_exp_minutes: i64,
    pub refresh_token_exp_days: i64,
    pub issuer: String,
}

impl JwtConfig {
    fn from_env() -> Self {
        Self {
            access_token_secret: env::var("ACCESS_TOKEN_SECRET")
                .unwrap_or_else(|_| "access_dev_key".into()),
            refresh_token_secret: env::var("REFRESH_TOKEN_SECRET")
                .unwrap_or_else(|_| "refresh_dev_key".into()),
            access_token_exp_minutes: 15,
            refresh_token_exp_days: 7,
            issuer: "idment_auth".into(),
        }
    }
}
