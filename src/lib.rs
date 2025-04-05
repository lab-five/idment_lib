pub mod error;
pub mod jwt;
pub mod models;
pub mod traits;

use config::{Config, File};
use std::path::Path;

pub fn load_config(path: Option<&str>) -> Result<models::AuthConfig, error::AuthError> {
    let mut builder = Config::builder();

    if let Some(p) = path {
        builder = builder.add_source(File::with_name(p));
    }

    builder
        .build()
        .map_err(|e| error::AuthError::ConfigError(e.to_string()))?
        .try_deserialize()
        .map_err(|e| error::AuthError::ConfigError(e.to_string()))
}
