use std::borrow::{Borrow, BorrowMut};

use drophub::{AccessToken, ClientId, ClientRole, EntityId, RoomError};
use serde_json::json;

use crate::server::room::types::Room;

/// Needs to drop after checks because validator holds lock to room.
#[derive(Debug)]
pub struct RoomValidator<'a, R> {
    token: &'a AccessToken,
    room: R,
}

pub trait RoomValidate {
    fn validate_revoke_invite(&self) -> Result<(), RoomError>;
    fn validate_kick(&self, client_id: ClientId) -> Result<(), RoomError>;
    fn validate_announce_entity(&self, entity_id: EntityId) -> Result<(), RoomError>;
    fn validate_remove_entity(&self, entity_id: EntityId) -> Result<(), RoomError>;
}

pub trait RoomMutValidate: RoomValidate {
    fn validate_invite(&mut self) -> Result<(), RoomError>;
}

impl<'a, R> RoomValidator<'a, R> {
    pub fn new(token: &'a AccessToken, room: R) -> Self {
        Self { token, room }
    }
}

impl<'a, R> RoomValidator<'a, R>
where
    R: Borrow<Room>,
{
    fn check_host_only(&self) -> Result<(), RoomError> {
        if self.token.role != ClientRole::Host {
            return Err(RoomError::PermissionDenied {
                client_id: self.token.client_id,
                room_id: self.token.room_id,
                details: Some(json!({ "client_role": self.token.role })),
            });
        }

        Ok(())
    }

    fn check_entity_owner(&self, entity_id: EntityId) -> Result<(), RoomError> {
        if !self
            .room
            .borrow()
            .is_entity_owner(entity_id, self.token.client_id)?
        {
            return Err(RoomError::PermissionDenied {
                client_id: self.token.client_id,
                room_id: self.token.room_id,
                details: Some(json!({ "entity_id": entity_id })),
            });
        }

        Ok(())
    }

    fn check_kick_yourself(&self, client_id: ClientId) -> Result<(), RoomError> {
        // TODO: kick yourself and switch roles with a random client?
        if self.token.client_id == client_id {
            return Err(RoomError::PermissionDenied {
                client_id: self.token.client_id,
                room_id: self.token.room_id,
                details: Some(json!("Host cannot kick itself")),
            });
        }

        Ok(())
    }

    fn check_entity_exists(&self, file_id: EntityId) -> Result<(), RoomError> {
        if self.room.borrow().is_file_exists(file_id) {
            return Err(RoomError::EntityAlreadyExists {
                entity_id: file_id,
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
                room_id: self.token.room_id,
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

    fn validate_announce_entity(&self, entity_id: EntityId) -> Result<(), RoomError> {
        self.check_entity_exists(entity_id)?;
        Ok(())
    }

    fn validate_remove_entity(&self, file_id: EntityId) -> Result<(), RoomError> {
        self.check_entity_owner(file_id)?;
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
