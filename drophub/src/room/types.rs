use std::collections::HashMap;

use base64::{engine::general_purpose::STANDARD as B64_ENGINE, Engine};
use jsonrpsee::SubscriptionMessage;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::JwtEncoded;

pub type Crc32Hash = u32;
pub type RoomId = u64;
pub type InviteId = String;
pub type FileId = Crc32Hash;
pub type ClientId = Uuid;
pub type DownloadProcId = Uuid;

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
    pub room_id: RoomId,
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
    pub checksum: Crc32Hash,
}

/// Base64 encoded bytes.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileData(Vec<u8>);

impl Serialize for FileData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let b64_enc = B64_ENGINE.encode(&self.0);
        b64_enc.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FileData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let b64_str = String::deserialize(deserializer)?;
        let b64_dec = B64_ENGINE.decode(b64_str).map_err(D::Error::custom)?;
        Ok(FileData(b64_dec))
    }
}

impl TryFrom<FileData> for SubscriptionMessage {
    type Error = serde_json::Error;

    fn try_from(f: FileData) -> Result<Self, Self::Error> {
        SubscriptionMessage::from_json(&f)
    }
}

impl TryFrom<&FileData> for SubscriptionMessage {
    type Error = serde_json::Error;

    fn try_from(f: &FileData) -> Result<Self, Self::Error> {
        SubscriptionMessage::from_json(f)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UploadRequest {
    pub download_id: DownloadProcId,
    pub file_id: FileId,
    pub block_idx: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientEvent {
    Init(JwtEncoded),
    RoomInfo(RoomInfo),
    UploadRequest(UploadRequest),
}

impl TryFrom<ClientEvent> for SubscriptionMessage {
    type Error = serde_json::Error;

    fn try_from(f: ClientEvent) -> Result<Self, Self::Error> {
        SubscriptionMessage::from_json(&f)
    }
}

impl TryFrom<&ClientEvent> for SubscriptionMessage {
    type Error = serde_json::Error;

    fn try_from(f: &ClientEvent) -> Result<Self, Self::Error> {
        SubscriptionMessage::from_json(f)
    }
}
