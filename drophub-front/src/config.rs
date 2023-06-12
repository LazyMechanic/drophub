use std::time::Duration;

use dotenvy_macro::dotenv;
use url::Url;

use crate::error::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Config {
    pub api_server_url: Url,
    pub init_timeout: Duration,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        Ok(Config {
            api_server_url: dotenv!("DROPHUB_FRONT_API_SERVER_URL").parse()?,
            init_timeout: humantime::parse_duration(&dotenv!("DROPHUB_FRONT_INIT_TIMEOUT"))?,
        })
    }
}
