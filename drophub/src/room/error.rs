use jsonrpsee::types::{ErrorObject, ErrorObjectOwned};
use serde_json::json;

use crate::{ClientId, FileId, InviteId, RoomId};

pub const COMMON_CODE: i32 = -40000;
pub const NOT_FOUND_CODE: i32 = -40001;
pub const PERMISSION_DENIED_CODE: i32 = -40002;

#[derive(Debug, thiserror::Error)]
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
    #[error("Other error")]
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

impl RoomError {
    fn jrpc_error_code(&self) -> i32 {
        match self {
            RoomError::RoomNotFound { .. } => NOT_FOUND_CODE,
            RoomError::ClientNotFound { .. } => NOT_FOUND_CODE,
            RoomError::InviteNotFound { .. } => NOT_FOUND_CODE,
            RoomError::FileNotFound { .. } => NOT_FOUND_CODE,
            RoomError::PermissionDenied { .. } => PERMISSION_DENIED_CODE,
            RoomError::RoomIsFull { .. } => COMMON_CODE,
            RoomError::DownloadYourOwnFileNotAllowed { .. } => COMMON_CODE,
            RoomError::Other(_) => COMMON_CODE,
        }
    }

    fn jrpc_error_msg(&self) -> String {
        self.to_string()
    }

    fn jrpc_error_data(&self) -> Option<serde_json::Value> {
        match self {
            RoomError::RoomNotFound { room_id } => Some(json!({ "room_id": room_id })),
            RoomError::ClientNotFound { client_id, room_id } => {
                Some(json!({ "client_id": client_id, "room_id": room_id }))
            }
            RoomError::InviteNotFound { invite_id, room_id } => {
                Some(json!({ "invite_id": invite_id, "room_id": room_id }))
            }
            RoomError::FileNotFound { file_id, room_id } => {
                Some(json!({ "file_id": file_id, "room_id": room_id }))
            }
            RoomError::PermissionDenied {
                client_id,
                room_id,
                details,
            } => Some(json!({
                "client_id": client_id,
                "room_id": room_id,
                "details": details,
            })),
            RoomError::RoomIsFull { room_id, capacity } => {
                Some(json!({ "room_id": room_id, "capacity": capacity }))
            }
            RoomError::DownloadYourOwnFileNotAllowed {
                client_id,
                file_id,
                room_id,
            } => Some(json!({
                "client_id": client_id,
                "file_id": file_id,
                "room_id": room_id,
            })),
            RoomError::Other(_) => None,
        }
    }
}
