use jsonrpsee::types::{ErrorObject, ErrorObjectOwned};
use serde::de::Error as _;

use crate::{EntityId, InvitePassphrase, PeerId, RoomId};

pub const COMMON_CODE: i32 = -40000;
pub const NOT_FOUND_CODE: i32 = -40001;
pub const PERMISSION_DENIED_CODE: i32 = -40002;

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Error {
    #[error("Room not found")]
    RoomNotFound { room_id: RoomId },
    #[error("Client not found")]
    PeerNotFound { peer_id: PeerId },
    #[error("Entity not found")]
    EntityNotFound {
        room_id: RoomId,
        entity_id: EntityId,
    },
    #[error("Permission denied")]
    PermissionDenied {
        room_id: Option<RoomId>,
        peer_id: PeerId,
        details: Option<serde_json::Value>,
    },
    #[error("Entity already exists")]
    EntityAlreadyExists {
        room_id: RoomId,
        entity_id: EntityId,
    },
    #[error("Peer is busy")]
    PeerIsBusy {
        peer_id: PeerId,
        details: Option<serde_json::Value>,
    },
    #[error("Same peer")]
    SamePeer {
        peer_id: PeerId,
        details: Option<serde_json::Value>,
    },
    #[error("Peer already connected")]
    PeerAlreadyConnected { peer_id: PeerId, room_id: RoomId },
    #[error("Invite not found")]
    InviteNotFound { invite_passphrase: InvitePassphrase },
    #[error("Mongodb error")]
    MongodbError {
        message: String,
        details: Option<serde_json::Value>,
    },
    #[error("Other error")]
    #[serde(with = "serde_other_error")]
    Other(#[from] anyhow::Error),
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(f: jsonwebtoken::errors::Error) -> Self {
        Error::Other(f.into())
    }
}

impl From<Error> for ErrorObjectOwned {
    fn from(f: Error) -> Self {
        let code = f.jrpc_error_code();
        let msg = f.jrpc_error_msg();
        let data = f.jrpc_error_data();
        ErrorObject::owned(code, msg, data)
    }
}

impl TryFrom<ErrorObjectOwned> for Error {
    type Error = serde_json::Error;

    fn try_from(f: ErrorObjectOwned) -> Result<Self, Self::Error> {
        let data = f
            .data()
            .ok_or_else(|| serde_json::Error::custom("Result does not contain 'data'"))?;
        serde_json::from_str(data.get())
    }
}

impl Error {
    fn jrpc_error_code(&self) -> i32 {
        match self {
            Error::RoomNotFound { .. } => NOT_FOUND_CODE,
            Error::PeerNotFound { .. } => NOT_FOUND_CODE,
            Error::EntityNotFound { .. } => NOT_FOUND_CODE,
            Error::PermissionDenied { .. } => PERMISSION_DENIED_CODE,
            Error::EntityAlreadyExists { .. } => COMMON_CODE,
            Error::PeerIsBusy { .. } => COMMON_CODE,
            Error::SamePeer { .. } => COMMON_CODE,
            Error::PeerAlreadyConnected { .. } => COMMON_CODE,
            Error::InviteNotFound { .. } => NOT_FOUND_CODE,
            Error::MongodbError { .. } => COMMON_CODE,
            Error::Other(_) => COMMON_CODE,
        }
    }

    fn jrpc_error_msg(&self) -> String {
        self.to_string()
    }

    fn jrpc_error_data(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self).ok()
    }
}

mod serde_other_error {
    use anyhow::anyhow;
    use serde::{self, ser::SerializeStruct, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(to_ser: &anyhow::Error, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("other", 1)?;
        s.serialize_field("details", &format!("{:#}", to_ser))?;
        s.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<anyhow::Error, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Deserialized {
            details: String,
        }

        let deserialized = Deserialized::deserialize(deserializer)?;
        Ok(anyhow!(deserialized.details))
    }
}
