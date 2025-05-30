use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Database connection string
    pub db_url: Url,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_url: Url::parse("postgres://apelle:apelle@localhost/apelle").unwrap(),
        }
    }
}
