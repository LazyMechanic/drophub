use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct JwtEncoded {
    pub access_token: String,
    pub refresh_token: String,
}
