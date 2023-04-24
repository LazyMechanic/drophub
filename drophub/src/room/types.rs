use std::collections::HashMap;

use base64::{engine::general_purpose::STANDARD as B64_ENGINE, Engine};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use time::OffsetDateTime;
use uuid::Uuid;

pub type RoomId = u64;
pub type InviteId = String;
pub type FileId = u64;
pub type ClientId = Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RoomOptions {
    pub encryption: bool,
    pub capacity: usize,
}

impl Default for RoomOptions {
    fn default() -> Self {
        Self {
            encryption: false,
            capacity: 5,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RoomInfo {
    pub host_id: ClientId,
    pub files: HashMap<FileId, FileMeta>,
    pub clients: Vec<ClientId>,
    pub invites: Vec<InviteId>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Invite {
    pub id: InviteId,
    pub room_id: RoomId,
    pub exp: OffsetDateTime,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileMeta {
    pub name: String,
    pub size: usize,
    /// CRC32 hash.
    pub checksum: u32,
}

/// Base64 encoded bytes.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct File(Vec<u8>);

impl Serialize for File {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let b64_enc = B64_ENGINE.encode(&self.0);
        b64_enc.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for File {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let b64_str = String::deserialize(deserializer)?;
        let b64_dec = B64_ENGINE.decode(b64_str).map_err(D::Error::custom)?;
        Ok(File(b64_dec))
    }
}
