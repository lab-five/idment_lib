use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Claims {
    #[validate(length(min = 1))]
    pub sub: String,

    #[validate(range(min = 1577836800))]
    pub exp: i64,

    #[validate(url)]
    pub iss: String,

    #[validate(length(min = 1))]
    pub aud: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    #[serde(default = "default_issuer")]
    pub issuer: String,

    #[serde(default = "default_audience")]
    pub audience: String,

    #[serde(default)]
    pub jwks_url: Option<String>,
}

fn default_issuer() -> String {
    "https://auth.yourdomain.com".into()
}

fn default_audience() -> String {
    "https://api.yourdomain.com".into()
}
