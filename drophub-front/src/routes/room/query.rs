use std::str::FromStr;

use serde::{de::Error, Deserialize, Deserializer, Serialize};

fn from_str<'de, D, S>(deserializer: D) -> Result<S, D::Error>
where
    D: Deserializer<'de>,
    S: FromStr,
{
    let s = <&str as Deserialize>::deserialize(deserializer)?;
    S::from_str(&s).map_err(|_| D::Error::custom("could not parse string"))
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum Query {
    Create(ActionCreate),
    Connect(ActionConnect),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActionCreate {
    #[serde(deserialize_with = "from_str")]
    pub encryption: bool,
    #[serde(deserialize_with = "from_str")]
    pub capacity: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActionConnect {
    #[serde(deserialize_with = "from_str")]
    pub room_id: u64,
    pub invite_password: String,
}
