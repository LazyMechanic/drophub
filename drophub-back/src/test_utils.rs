use lazy_static::lazy_static;

use crate::config::Config;

pub fn test_config() -> Config {
    lazy_static! {
        static ref CONFIG: Config = {
            const CONFIG_TEXT: &str = include_str!("../tests/config.yaml");
            serde_yaml::from_str(CONFIG_TEXT).unwrap()
        };
    }

    CONFIG.clone()
}
