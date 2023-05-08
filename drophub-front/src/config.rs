use dotenvy_macro::dotenv;
use url::Url;
use yew::Properties;

use crate::error::Error;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Config {
    pub api_root_url: Url,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        Ok(Config {
            api_root_url: dotenv!("DROPHUB_FRONT_API_SERVER_URL").parse()?,
        })
    }
}
