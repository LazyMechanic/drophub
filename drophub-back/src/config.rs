use std::{net::SocketAddr, path::Path, time::Duration};

use config as config_lib;
use dotenv::dotenv;
use mongodb::options::{Credential, ServerAddress};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub mongodb: MongodbConfig,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_addr: SocketAddr,
    pub secret: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MongodbConfig {
    pub uri: String,
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
