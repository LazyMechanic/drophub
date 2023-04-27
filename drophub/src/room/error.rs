use jsonrpsee::types::{ErrorObject, ErrorObjectOwned};
use serde::de::Error;

use crate::{ClientId, DownloadProcId, FileId, InviteId, RoomId};

pub const COMMON_CODE: i32 = -40000;
pub const NOT_FOUND_CODE: i32 = -40001;
pub const PERMISSION_DENIED_CODE: i32 = -40002;
pub const INVALID_FILE_BLOCK_SIZE_CODE: i32 = -40003;

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum RoomError {
    #[error("Room not found")]
    RoomNotFound { room_id: RoomId },
    #[error("Client not found")]
    ClientNotFound {
        client_id: ClientId,
        room_id: RoomId,
    },
    #[error("Invite not found")]
    InviteNotFound {
        invite_id: InviteId,
        room_id: RoomId,
    },
    #[error("File not found")]
    FileNotFound { file_id: FileId, room_id: RoomId },
    #[error("Download process not found")]
    DownloadProcessNotFound {
        download_proc_id: DownloadProcId,
        room_id: RoomId,
    },
    #[error("Permission denied")]
    PermissionDenied {
        client_id: ClientId,
        room_id: RoomId,
        details: Option<serde_json::Value>,
    },
    #[error("Room is full")]
    RoomIsFull { room_id: RoomId, capacity: usize },
    #[error("Download your own file are not allowed")]
    DownloadYourOwnFileNotAllowed {
        client_id: ClientId,
        file_id: FileId,
        room_id: RoomId,
    },
    #[error("File already exists")]
    FileAlreadyExists { file_id: FileId, room_id: RoomId },
    #[error("Invalid file block size")]
    InvalidFileBlockSize {
        file_id: FileId,
        recv_block_size: usize,
        exp_block_size: usize,
        room_id: RoomId,
    },
    #[error("Other error")]
    #[serde(with = "serde_other_error")]
    Other(#[from] anyhow::Error),
}

impl From<RoomError> for ErrorObjectOwned {
    fn from(f: RoomError) -> Self {
        let code = f.jrpc_error_code();
        let msg = f.jrpc_error_msg();
        let data = f.jrpc_error_data();
        ErrorObject::owned(code, msg, data)
    }
}

impl TryFrom<ErrorObjectOwned> for RoomError {
    type Error = serde_json::Error;

    fn try_from(f: ErrorObjectOwned) -> Result<Self, Self::Error> {
        let data = f
            .data()
            .ok_or_else(|| serde_json::Error::custom("Result does not contain 'data'"))?;
        serde_json::from_str(data.get())
    }
}

impl RoomError {
    fn jrpc_error_code(&self) -> i32 {
        match self {
            RoomError::RoomNotFound { .. } => NOT_FOUND_CODE,
            RoomError::ClientNotFound { .. } => NOT_FOUND_CODE,
            RoomError::InviteNotFound { .. } => NOT_FOUND_CODE,
            RoomError::FileNotFound { .. } => NOT_FOUND_CODE,
            RoomError::DownloadProcessNotFound { .. } => NOT_FOUND_CODE,
            RoomError::PermissionDenied { .. } => PERMISSION_DENIED_CODE,
            RoomError::RoomIsFull { .. } => COMMON_CODE,
            RoomError::DownloadYourOwnFileNotAllowed { .. } => COMMON_CODE,
            RoomError::FileAlreadyExists { .. } => COMMON_CODE,
            RoomError::InvalidFileBlockSize { .. } => INVALID_FILE_BLOCK_SIZE_CODE,
            RoomError::Other(_) => COMMON_CODE,
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
