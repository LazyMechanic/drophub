use std::{net::SocketAddr, path::Path, time::Duration};

use config as config_lib;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_addr: SocketAddr,
    pub jwt: JwtConfig,
    #[serde(with = "humantime_serde")]
    pub invite_duration: Duration,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct JwtConfig {
    pub token_secret: String,
    #[serde(with = "humantime_serde")]
    pub access_token_duration: Duration,
    #[serde(with = "humantime_serde")]
    pub refresh_token_duration: Duration,
}

impl Config {
    pub fn new<P: AsRef<Path>>(path: Option<P>) -> anyhow::Result<Self> {
        dotenv().ok();

        let mut cfg = config_lib::Config::builder();
        if let Some(path) = path {
            let path = path.as_ref().to_str().ok_or_else(|| {
                config_lib::ConfigError::Message("invalid path utf-8 encoding".to_owned())
            })?;
            cfg = cfg.add_source(config_lib::File::with_name(path));
        }
        cfg = cfg.add_source(
            config_lib::Environment::default()
                .prefix("DROPHUB")
                .separator("__")
                .ignore_empty(true),
        );

        let cfg = cfg.build()?;
        Ok(cfg.try_deserialize()?)
    }
}
