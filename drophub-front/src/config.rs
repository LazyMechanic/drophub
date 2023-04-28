use dotenvy_macro::dotenv;
use url::Url;
use yew::Properties;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Config {
    pub api_root_url: Url,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Config {
            api_root_url: dotenv!("DROPHUB_FRONT_API_SERVER_URL").parse()?,
        })
    }
}
