use std::borrow::Cow;

use jsonrpsee::types::{
    error::{CallError, CALL_EXECUTION_FAILED_CODE, INVALID_PARAMS_CODE},
    ErrorObject, ErrorObjectOwned,
};
use serde_json::json;

use crate::{ClientId, InviteId, RoomId};

pub const NOT_FOUND_CODE: i32 = -40000;
pub const PERMISSION_DENIED_CODE: i32 = -40001;

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
    #[error("Permission denied")]
    PermissionDenied {
        client_id: ClientId,
        room_id: RoomId,
        details: Option<serde_json::Value>,
    },
    #[error("Other error")]
    Other(#[from] anyhow::Error),
    #[error(transparent)]
    Builtin(#[from] CallError),
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
            RoomError::PermissionDenied { .. } => PERMISSION_DENIED_CODE,
            RoomError::Other(_) => CALL_EXECUTION_FAILED_CODE,
            RoomError::Builtin(err) => match err {
                CallError::InvalidParams(_) => INVALID_PARAMS_CODE,
                CallError::Failed(_) => CALL_EXECUTION_FAILED_CODE,
                CallError::Custom(err) => err.code(),
            },
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
            RoomError::PermissionDenied {
                client_id,
                room_id,
                details,
            } => Some(json!({
                "client_id": client_id,
                "room_id": room_id,
                "details": details,
            })),
            RoomError::Other(_) => None,
            RoomError::Builtin(err) => match err {
                CallError::InvalidParams(_) => None,
                CallError::Failed(_) => None,
                // TODO: optimize allocation
                CallError::Custom(err) => {
                    err.data().and_then(|v| serde_json::from_str(v.get()).ok())
                }
            },
        }
    }
}
