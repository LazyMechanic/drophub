use std::str::FromStr;

use serde::{de::Error, Deserialize, Deserializer, Serialize};
use yew::prelude::*;

fn from_str<'de, D, S>(deserializer: D) -> Result<S, D::Error>
where
    D: Deserializer<'de>,
    S: FromStr,
{
    let s = <&str as Deserialize>::deserialize(deserializer)?;
    S::from_str(&s).map_err(|_| D::Error::custom("could not parse string"))
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Query {
    #[serde(deserialize_with = "from_str")]
    pub encryption: bool,
    #[serde(deserialize_with = "from_str")]
    pub capacity: usize,
}

#[function_component(CreateRoom)]
pub fn create_room() -> Html {
    todo!()
}
