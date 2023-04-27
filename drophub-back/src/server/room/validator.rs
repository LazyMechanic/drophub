use std::borrow::{Borrow, BorrowMut};

use drophub::{ClientId, FileId, RoomError};
use serde_json::json;

use crate::{
    jwt::{ClientRole, Jwt},
    server::room::types::Room,
};

/// Needs to drop after checks because validator holds lock to room.
#[derive(Debug)]
pub struct RoomValidator<'a, R> {
    jwt: &'a Jwt,
    room: R,
}

pub trait RoomValidate {
    fn validate_revoke_invite(&self) -> Result<(), RoomError>;
    fn validate_kick(&self, client_id: ClientId) -> Result<(), RoomError>;
    fn validate_announce_file(&self, file_id: FileId) -> Result<(), RoomError>;
    fn validate_remove_file(&self, file_id: FileId) -> Result<(), RoomError>;
    fn validate_upload_file(&self) -> Result<(), RoomError>;
    fn validate_sub_download(&self, file_id: FileId) -> Result<(), RoomError>;
}

pub trait RoomMutValidate: RoomValidate {
    fn validate_invite(&mut self) -> Result<(), RoomError>;
}

impl<'a, R> RoomValidator<'a, R> {
    pub fn new(jwt: &'a Jwt, room: R) -> Self {
        Self { jwt, room }
    }
}

impl<'a, R> RoomValidator<'a, R>
where
    R: Borrow<Room>,
{
    fn check_host_only(&self) -> Result<(), RoomError> {
        if self.jwt.access_token.role != ClientRole::Host {
            return Err(RoomError::PermissionDenied {
                client_id: self.jwt.access_token.client_id,
                room_id: self.jwt.access_token.room_id,
                details: Some(json!({ "client_role": self.jwt.access_token.role })),
            });
        }

        Ok(())
    }

    fn check_file_owner(&self, file_id: FileId) -> Result<(), RoomError> {
        if !self
            .room
            .borrow()
            .is_file_owner(file_id, self.jwt.access_token.client_id)?
        {
            return Err(RoomError::PermissionDenied {
                client_id: self.jwt.access_token.client_id,
                room_id: self.jwt.access_token.room_id,
                details: Some(json!({ "file_id": file_id })),
            });
        }

        Ok(())
    }

    fn check_download_file(&self, file_id: FileId) -> Result<(), RoomError> {
        if self
            .room
            .borrow()
            .is_file_owner(file_id, self.jwt.access_token.client_id)?
        {
            return Err(RoomError::DownloadYourOwnFileNotAllowed {
                client_id: self.jwt.access_token.client_id,
                file_id,
                room_id: self.jwt.access_token.room_id,
            });
        }

        Ok(())
    }

    fn check_kick_yourself(&self, client_id: ClientId) -> Result<(), RoomError> {
        // TODO: kick yourself and switch roles with a random client?
        if self.jwt.access_token.client_id == client_id {
            return Err(RoomError::PermissionDenied {
                client_id: self.jwt.access_token.client_id,
                room_id: self.jwt.access_token.room_id,
                details: Some(json!("Host cannot kick itself")),
            });
        }

        Ok(())
    }

    fn check_file_exists(&self, file_id: FileId) -> Result<(), RoomError> {
        if self.room.borrow().is_file_exists(file_id) {
            return Err(RoomError::FileAlreadyExists {
                file_id,
                room_id: self.room.borrow().id(),
            });
        }

        Ok(())
    }
}

impl<'a, R> RoomValidator<'a, R>
where
    R: BorrowMut<Room>,
{
    fn check_capacity(&mut self) -> Result<(), RoomError> {
        if self.room.borrow_mut().is_full() {
            return Err(RoomError::RoomIsFull {
                room_id: self.jwt.access_token.room_id,
                capacity: self.room.borrow().capacity(),
            });
        }

        Ok(())
    }
}

impl<R> RoomValidate for RoomValidator<'_, R>
where
    R: Borrow<Room>,
{
    fn validate_revoke_invite(&self) -> Result<(), RoomError> {
        self.check_host_only()?;
        Ok(())
    }

    fn validate_kick(&self, client_id: ClientId) -> Result<(), RoomError> {
        self.check_host_only()?;
        self.check_kick_yourself(client_id)?;
        Ok(())
    }

    fn validate_announce_file(&self, file_id: FileId) -> Result<(), RoomError> {
        self.check_file_exists(file_id)?;
        Ok(())
    }

    fn validate_remove_file(&self, file_id: FileId) -> Result<(), RoomError> {
        self.check_file_owner(file_id)?;
        Ok(())
    }

    fn validate_upload_file(&self) -> Result<(), RoomError> {
        Ok(())
    }

    fn validate_sub_download(&self, file_id: FileId) -> Result<(), RoomError> {
        self.check_download_file(file_id)?;
        Ok(())
    }
}

impl<R> RoomMutValidate for RoomValidator<'_, R>
where
    R: BorrowMut<Room>,
{
    fn validate_invite(&mut self) -> Result<(), RoomError> {
        self.check_host_only()?;
        self.check_capacity()?;
        Ok(())
    }
}
