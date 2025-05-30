use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
/// Configuration of the app for serving it
pub struct ServeConfig {
    /// Socket address
    pub socket: SocketConfig,

    /// Show the starting banner
    pub banner: bool,
}

impl ServeConfig {
    pub fn default(service_default_port: u16) -> Self {
        Self {
            socket: SocketConfig::default(service_default_port),
            banner: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// Address to bind the app to
#[serde(untagged)]
pub enum SocketConfig {
    Compact(String),
    Large {
        /// The ip
        ip: String,
        // The port
        port: u16,
    },
}
impl SocketConfig {
    fn default(service_default_port: u16) -> Self {
        Self::Large {
            ip: "127.0.0.1".to_owned(),
            port: service_default_port,
        }
    }
}

impl Display for SocketConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketConfig::Compact(s) => write!(f, "{s}"),
            SocketConfig::Large { ip, port } => write!(f, "{ip}:{port}"),
        }
    }
}
