use std::{
    collections::HashSet,
    fmt::{Debug, Formatter},
    ops::Add,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use anyhow::anyhow;
use drophub::{
    new_entity_id, ClientId, ClientRole, EntityId, EntityMeta, Invite, InvitePassword, RoomError,
    RoomId, RoomInfo, RoomOptions,
};
use indexmap::IndexMap;
use passwords::PasswordGenerator;
use replace_with::replace_with_or_default;
use time::OffsetDateTime;
use tokio::sync::broadcast;
use ttl_cache::TtlCache;
use uuid::Uuid;

use crate::config::RoomConfig;

pub struct Room {
    id: RoomId,
    host_id: ClientId,
    clients: IndexMap<ClientId, Client>,
    entities: IndexMap<EntityId, Entity>,
    invites: TtlCache<InvitePassword, Invite>,
    options: RoomOptions,
    invite_ttl: Duration,
    info_tx: broadcast::Sender<RoomInfo>,
    invite_gen: PasswordGenerator,
}

impl Debug for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Room")
            .field("id", &self.id)
            .field("host_id", &self.host_id)
            .field("clients", &self.clients)
            .field("entities", &self.entities)
            .field("invites", &"{ ... }")
            .field("options", &self.options)
            .field("invite_ttl", &self.invite_ttl)
            .field("info_tx", &"{ ... }")
            .field("invite_gen", &self.invite_gen)
            .finish()
    }
}

impl Room {
    pub fn new(options: RoomOptions, host: Client, cfg: &RoomConfig) -> Self {
        let (info_tx, _) = broadcast::channel(5);

        Self {
            id: Self::next_id(),
            host_id: host.id,
            clients: {
                let mut c = IndexMap::new();
                c.insert(host.id, host);
                c
            },
            entities: Default::default(),
            invites: TtlCache::new(usize::MAX),
            options,
            invite_ttl: cfg.invite_ttl,
            info_tx,
            invite_gen: PasswordGenerator {
                length: 8,
                numbers: true,
                lowercase_letters: true,
                uppercase_letters: false,
                symbols: false,
                spaces: false,
                exclude_similar_characters: true,
                strict: true,
            },
        }
    }

    pub fn id(&self) -> RoomId {
        self.id
    }

    pub fn host_id(&self) -> ClientId {
        self.host_id
    }

    pub fn capacity(&self) -> usize {
        self.options.capacity
    }

    pub fn broadcast_info(&mut self) -> Result<(), RoomError> {
        let room_info = self.info();
        tracing::info!(?room_info, "Broadcast room info");

        self.info_tx
            .send(room_info)
            .map_err(|_| anyhow!("Room info channel closed"))?;

        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RoomInfo> {
        self.info_tx.subscribe()
    }

    pub fn generate_invite(&mut self) -> Result<Invite, RoomError> {
        loop {
            let invite_password = self
                .invite_gen
                .generate_one()
                .map_err(|err| anyhow!("{}", err))?;

            if !self.invites.contains_key(&invite_password) {
                let invite = Invite {
                    room_id: self.id,
                    password: invite_password.clone(),
                    exp: OffsetDateTime::now_utc().add(self.invite_ttl),
                };
                // TODO: remove invite forced after ttl and broadcast updated info
                self.invites
                    .insert(invite_password, invite.clone(), self.invite_ttl);
                break Ok(invite);
            }
        }
    }

    pub fn revoke_invite(&mut self, invite_id: InvitePassword) -> Result<(), RoomError> {
        if self.invites.remove(&invite_id).is_none() {
            return Err(RoomError::InviteNotFound {
                room_id: self.id,
                invite_password: invite_id,
            });
        }

        Ok(())
    }

    pub fn add_entity(&mut self, entity: Entity) -> Result<(), RoomError> {
        let client = self.get_client_mut(entity.owner)?;

        client.files.insert(entity.id);
        self.entities.insert(entity.id, entity);

        Ok(())
    }

    pub fn remove_entity(&mut self, entity_id: EntityId) -> Result<(), RoomError> {
        let file = self
            .entities
            .remove(&entity_id)
            .ok_or(RoomError::EntityNotFound {
                room_id: self.id,
                entity_id,
            })?;

        let client = self.get_client_mut(file.owner)?;
        client.files.remove(&entity_id);

        Ok(())
    }

    pub fn add_client(
        &mut self,
        client: Client,
        invite_id: InvitePassword,
    ) -> Result<broadcast::Receiver<RoomInfo>, RoomError> {
        let _ = self
            .invites
            .remove(&invite_id)
            .ok_or(RoomError::InviteNotFound {
                invite_password: invite_id,
                room_id: self.id,
            })?;

        // Add connected client
        self.clients.insert(client.id, client);
        Ok(self.info_tx.subscribe())
    }

    pub fn remove_client(&mut self, client_id: ClientId) -> Result<(), RoomError> {
        // Remove client from list
        self.clients.remove(&client_id);

        // Remove all client-owned files
        // TODO: don't delete the file if any other client has it
        replace_with_or_default(&mut self.entities, |room_files| {
            room_files
                .into_iter()
                .filter(|(_, entity)| entity.owner != client_id)
                .collect()
        });
        Ok(())
    }

    pub fn is_entity_owner(
        &self,
        entity_id: EntityId,
        client_id: ClientId,
    ) -> Result<bool, RoomError> {
        let entity = self.get_entity(entity_id)?;
        Ok(client_id == entity.owner)
    }

    pub fn is_file_exists(&self, file_id: EntityId) -> bool {
        self.entities.contains_key(&file_id)
    }

    pub fn is_full(&mut self) -> bool {
        // TODO: optimize `self.invites.iter().count()`
        self.invites.iter().count() + self.clients.len() >= self.options.capacity
    }

    fn next_id() -> RoomId {
        static ID: AtomicU64 = AtomicU64::new(0);
        ID.fetch_add(1, Ordering::Relaxed)
    }

    fn info(&mut self) -> RoomInfo {
        RoomInfo {
            id: self.id,
            host: self.host_id,
            entities: self
                .entities
                .iter()
                .map(|(&file_id, file)| (file_id, file.meta.clone()))
                .collect(),
            clients: self
                .clients
                .iter()
                .map(|(&client_id, _)| client_id)
                .collect(),
            invites: self
                .invites
                .iter()
                .map(|(invite_id, _)| invite_id.clone())
                .collect(),
            options: self.options.clone(),
        }
    }

    fn get_entity(&self, entity_id: EntityId) -> Result<&Entity, RoomError> {
        self.entities
            .get(&entity_id)
            .ok_or(RoomError::EntityNotFound {
                room_id: self.id,
                entity_id,
            })
    }

    fn get_client_mut(&mut self, client_id: ClientId) -> Result<&mut Client, RoomError> {
        self.clients
            .get_mut(&client_id)
            .ok_or(RoomError::ClientNotFound {
                room_id: self.id,
                client_id,
            })
    }
}

#[derive(Debug)]
pub struct Client {
    id: ClientId,
    role: ClientRole,
    files: HashSet<EntityId>,
}

impl Client {
    pub fn new(role: ClientRole) -> Self {
        Self {
            id: Self::next_id(),
            role,
            files: Default::default(),
        }
    }

    pub fn id(&self) -> ClientId {
        self.id
    }

    pub fn role(&self) -> ClientRole {
        self.role
    }

    fn next_id() -> ClientId {
        Uuid::new_v4()
    }
}

#[derive(Debug)]
pub struct Entity {
    id: EntityId,
    meta: EntityMeta,
    owner: ClientId,
}

impl Entity {
    pub fn new(owner: ClientId, meta: EntityMeta) -> Self {
        Self {
            id: new_entity_id(owner, meta.name()),
            meta,
            owner,
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }
}
