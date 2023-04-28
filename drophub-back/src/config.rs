use std::{net::SocketAddr, path::Path, time::Duration};

use config as config_lib;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub room: RoomConfig,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_addr: SocketAddr,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RoomConfig {
    pub jwt: JwtConfig,
    #[serde(with = "humantime_serde")]
    pub invite_ttl: Duration,
    /// File block size in bytes.
    #[serde(default = "RoomConfig::def_block_size")]
    pub block_size: usize,
}

impl RoomConfig {
    fn def_block_size() -> usize {
        8192
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct JwtConfig {
    pub token_secret: String,
    #[serde(default, with = "humantime_serde")]
    pub access_token_ttl: Option<Duration>,
    #[serde(default, with = "humantime_serde")]
    pub refresh_token_ttl: Option<Duration>,
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
                .prefix("DROPHUB_BACK")
                .separator("__")
                .ignore_empty(true),
        );

        let cfg = cfg.build()?;
        Ok(cfg.try_deserialize()?)
    }
}
