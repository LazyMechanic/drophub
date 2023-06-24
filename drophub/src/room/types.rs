use indexmap::IndexMap;
#[cfg(feature = "rpc-server")]
use jsonrpsee::SubscriptionMessage;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::RoomError;

pub type Crc32Hash = u32;
pub type RoomId = u64;
pub type EntityId = Crc32Hash;
pub type ClientId = Uuid;
pub type InvitePassword = String;
pub type AccessTokenEncoded = String;

pub fn new_client_id() -> ClientId {
    Uuid::new_v4()
}

pub fn new_entity_id(client_id: ClientId, entity_name: &str) -> EntityId {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(client_id.as_bytes());
    hasher.update(entity_name.as_bytes());
    hasher.finalize()
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RoomOptions {
    pub encryption: bool,
    pub capacity: usize,
}

impl Default for RoomOptions {
    fn default() -> Self {
        Self {
            encryption: false,
            capacity: 2,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RoomInfo {
    pub id: RoomId,
    pub host: ClientId,
    pub entities: IndexMap<EntityId, EntityMeta>,
    pub clients: Vec<ClientId>,
    pub invites: Vec<InvitePassword>,
    pub options: RoomOptions,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Invite {
    pub password: InvitePassword,
    pub room_id: RoomId,
    pub exp: OffsetDateTime,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityKind {
    File,
    Text,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum EntityMeta {
    File(FileMeta),
    Text(TextMeta),
}

impl EntityMeta {
    pub fn kind(&self) -> EntityKind {
        match self {
            EntityMeta::File(_) => EntityKind::File,
            EntityMeta::Text(_) => EntityKind::Text,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            EntityMeta::File(m) => &m.name,
            EntityMeta::Text(m) => &m.name,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileMeta {
    pub name: String,
    pub size: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TextMeta {
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoomEvent {
    Init {
        token: AccessTokenEncoded,
        client_id: ClientId,
        client_role: ClientRole,
    },
    RoomInfo(RoomInfo),
}

#[cfg(feature = "rpc-server")]
impl TryFrom<RoomEvent> for SubscriptionMessage {
    type Error = serde_json::Error;

    fn try_from(f: RoomEvent) -> Result<Self, Self::Error> {
        SubscriptionMessage::from_json(&f)
    }
}

#[cfg(feature = "rpc-server")]
impl TryFrom<&RoomEvent> for SubscriptionMessage {
    type Error = serde_json::Error;

    fn try_from(f: &RoomEvent) -> Result<Self, Self::Error> {
        SubscriptionMessage::from_json(f)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientRole {
    Host,
    Guest,
}

/// Token for accessing resources.
/// After expired needs to refresh access token via refresh token.
///
/// Can be encoded to JWT format `"header.payload.signature"`.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccessToken {
    pub client_id: ClientId,
    pub room_id: RoomId,
    pub role: ClientRole,
    #[serde(with = "jwt_numeric_date")]
    pub exp: Option<OffsetDateTime>,
}

impl AccessToken {
    /// Encodes token to JWT format.
    pub fn encode(&self, secret: &str) -> Result<String, RoomError> {
        let tok = jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        tracing::debug!("encoded access token: {:?}", tok);
        Ok(tok)
    }

    /// Decodes token from JWT format.
    pub fn decode(token: &str, secret: &str) -> Result<Self, RoomError> {
        let validation = {
            let mut v = Validation::default();
            v.set_required_spec_claims::<&str>(&[]);
            v
        };

        let tok = jsonwebtoken::decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )
        .map(|token_data| token_data.claims)?;

        tracing::debug!("decoded access token: {:?}", tok);
        Ok(tok)
    }

    pub fn create_span(&self) -> tracing::Span {
        tracing::info_span!(parent: tracing::Span::current(),
            "jwt",
            "room.id" = ?self.room_id,
            "client.id" = ?self.client_id,
            "client.role" = ?self.role,
        )
    }
}

mod jwt_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            None => serializer.serialize_none(),
            Some(date) => {
                let timestamp = date.unix_timestamp();
                serializer.serialize_i64(timestamp)
            }
        }
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let Some(ts ) = Option::<i64>::deserialize(deserializer)? else { return Ok(None) };
        let dt = OffsetDateTime::from_unix_timestamp(ts)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))?;

        Ok(Some(dt))
    }
}
