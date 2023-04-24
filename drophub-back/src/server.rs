use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter},
    ops::Add,
    sync::atomic::{AtomicU64, Ordering},
};

use anyhow::anyhow;
use dashmap::DashMap;
use drophub::{
    ClientId, File, FileId, FileMeta, Invite, InviteId, JwtEncoded, RoomError, RoomId, RoomOptions,
    RoomRpcServer,
};
use jsonrpsee_core::async_trait;
use passwords::PasswordGenerator;
use serde_json::json;
use time::OffsetDateTime;
use tracing::instrument;
use ttl_cache::TtlCache;
use uuid::Uuid;

use crate::{
    config::{Config, ServerConfig},
    jwt::{AccessToken, ClientRole, Jwt, RefreshToken},
};

#[derive(Debug)]
pub struct RpcServer {
    rooms: DashMap<RoomId, Room>,
    cfg: ServerConfig,
}

struct Room {
    host_id: ClientId,
    clients: HashMap<ClientId, Client>,
    files: HashMap<FileId, FileMeta>,
    invites: TtlCache<InviteId, Invite>,
    encryption: bool,
    capacity: usize,
}

impl Debug for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Room")
            .field("host_id", &self.host_id)
            .field("clients", &self.clients)
            .field("files", &self.files)
            .field("invites", &"{ ... }")
            .field("encryption", &self.encryption)
            .field("capacity", &self.capacity)
            .finish()
    }
}

impl Room {
    pub fn new(options: RoomOptions, host: (ClientId, Client)) -> (RoomId, Self) {
        (
            Self::next_id(),
            Self {
                host_id: host.0,
                clients: {
                    let mut c = HashMap::new();
                    c.insert(host.0, host.1);
                    c
                },
                files: Default::default(),
                invites: TtlCache::new(usize::MAX),
                encryption: options.encryption,
                capacity: options.capacity,
            },
        )
    }

    pub fn next_id() -> RoomId {
        static ID: AtomicU64 = AtomicU64::new(0);
        ID.fetch_add(1, Ordering::Relaxed)
    }
}

#[derive(Debug)]
struct Client {
    owned_files: HashSet<FileId>,
    role: ClientRole,
}

impl Client {
    pub fn new(role: ClientRole) -> (ClientId, Self) {
        (
            Self::next_id(),
            Self {
                owned_files: Default::default(),
                role,
            },
        )
    }

    pub fn next_id() -> ClientId {
        Uuid::new_v4()
    }
}

impl RpcServer {
    pub fn new(cfg: &Config) -> Self {
        Self {
            rooms: Default::default(),
            cfg: cfg.server.clone(),
        }
    }

    fn generate_invite(&self, room_id: RoomId) -> Result<Invite, RoomError> {
        let id_gen = PasswordGenerator {
            length: 8,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: false,
            symbols: false,
            spaces: false,
            exclude_similar_characters: true,
            strict: true,
        };

        let invite_id = id_gen.generate_one().map_err(|err| anyhow!("{}", err))?;

        Ok(Invite {
            id: invite_id,
            room_id,
            exp: OffsetDateTime::now_utc().add(self.cfg.invite_duration),
        })
    }

    fn check_host_only(&self, jwt: &Jwt) -> Result<(), RoomError> {
        if jwt.access_token.role != ClientRole::Host {
            return Err(RoomError::PermissionDenied {
                client_id: jwt.access_token.client_id,
                room_id: jwt.access_token.room_id,
                details: Some(json!({ "client_role": jwt.access_token.role })),
            });
        }

        Ok(())
    }

    fn check_file_owner(&self, jwt: &Jwt, file_id: FileId) -> Result<(), RoomError> {
        let room =
            self.rooms
                .get(&jwt.access_token.room_id)
                .ok_or_else(|| RoomError::RoomNotFound {
                    room_id: jwt.access_token.room_id,
                })?;

        let client = room
            .clients
            .get(&jwt.access_token.client_id)
            .ok_or_else(|| RoomError::ClientNotFound {
                client_id: jwt.access_token.client_id,
                room_id: jwt.access_token.room_id,
            })?;

        if !client.owned_files.contains(&file_id) {
            return Err(RoomError::PermissionDenied {
                client_id: jwt.access_token.client_id,
                room_id: jwt.access_token.room_id,
                details: Some(json!({ "file_id": file_id })),
            });
        }

        Ok(())
    }

    fn check_jwt(&self, jwt: &Jwt) -> Result<(), RoomError> {
        if jwt.access_token.exp > OffsetDateTime::now_utc() {
            return Err(RoomError::PermissionDenied {
                client_id: jwt.access_token.client_id,
                room_id: jwt.access_token.room_id,
                details: Some(json!({ "details": "JWT access token expired" })),
            });
        }

        Ok(())
    }
}

#[async_trait]
impl RoomRpcServer for RpcServer {
    #[instrument(skip(self), err(Debug))]
    async fn create(&self, options: RoomOptions) -> Result<JwtEncoded, RoomError> {
        let (client_id, client) = Client::new(ClientRole::Host);
        let client_role = client.role;
        let (room_id, room) = Room::new(options, (client_id, client));

        let host = Jwt {
            access_token: AccessToken {
                client_id,
                room_id,
                role: client_role,
                exp: OffsetDateTime::now_utc().add(self.cfg.jwt.access_token_duration),
            },
            refresh_token: RefreshToken {
                token: Uuid::new_v4(),
                exp: OffsetDateTime::now_utc().add(self.cfg.jwt.refresh_token_duration),
            },
        }
        .encode(&self.cfg.jwt.token_secret)?;

        self.rooms.insert(room_id, room);

        Ok(host)
    }

    #[instrument(skip(self), err(Debug))]
    async fn connect(&self, room_id: RoomId, invite_id: InviteId) -> Result<JwtEncoded, RoomError> {
        let (client_id, client) = Client::new(ClientRole::Guest);

        let guest = Jwt {
            access_token: AccessToken {
                client_id,
                room_id,
                role: client.role,
                exp: OffsetDateTime::now_utc().add(self.cfg.jwt.access_token_duration),
            },
            refresh_token: RefreshToken {
                token: Uuid::new_v4(),
                exp: OffsetDateTime::now_utc().add(self.cfg.jwt.refresh_token_duration),
            },
        }
        .encode(&self.cfg.jwt.token_secret)?;

        let mut room = self
            .rooms
            .get_mut(&room_id)
            .ok_or_else(|| RoomError::RoomNotFound { room_id })?;

        let _ = room
            .invites
            .remove(&invite_id)
            .ok_or_else(|| RoomError::InviteNotFound { invite_id, room_id })?;

        room.clients.insert(client_id, client);

        // TODO: broadcast updated room info

        Ok(guest)
    }

    #[instrument(skip(self, host_jwt), err(Debug))]
    async fn invite(&self, host_jwt: JwtEncoded) -> Result<Invite, RoomError> {
        let host_jwt = Jwt::decode(&host_jwt, &self.cfg.jwt.token_secret)?;
        self.check_jwt(&host_jwt)?;
        self.check_host_only(&host_jwt)?;

        let mut room = self
            .rooms
            .get_mut(&host_jwt.access_token.room_id)
            .ok_or_else(|| RoomError::RoomNotFound {
                room_id: host_jwt.access_token.room_id,
            })?;

        let invite = self.generate_invite(host_jwt.access_token.room_id)?;
        room.invites
            .insert(invite.id.clone(), invite.clone(), self.cfg.invite_duration);

        Ok(invite)
    }

    #[instrument(skip(self, host_jwt), err(Debug))]
    async fn revoke_invite(
        &self,
        host_jwt: JwtEncoded,
        invite_id: InviteId,
    ) -> Result<(), RoomError> {
        let host_jwt = Jwt::decode(&host_jwt, &self.cfg.jwt.token_secret)?;
        self.check_jwt(&host_jwt)?;
        self.check_host_only(&host_jwt)?;

        let mut room = self
            .rooms
            .get_mut(&host_jwt.access_token.room_id)
            .ok_or_else(|| RoomError::RoomNotFound {
                room_id: host_jwt.access_token.room_id,
            })?;

        let _ = room
            .invites
            .remove(&invite_id)
            .ok_or_else(|| RoomError::InviteNotFound {
                invite_id,
                room_id: host_jwt.access_token.room_id,
            })?;

        Ok(())
    }

    #[instrument(skip(self, host_jwt), err(Debug))]
    async fn kick(&self, host_jwt: JwtEncoded, client_id: ClientId) -> Result<(), RoomError> {
        let host_jwt = Jwt::decode(&host_jwt, &self.cfg.jwt.token_secret)?;
        self.check_jwt(&host_jwt)?;
        self.check_host_only(&host_jwt)?;

        // TODO: kick yourself and switch roles with a random client?
        if host_jwt.access_token.client_id == client_id {
            return Err(RoomError::PermissionDenied {
                client_id,
                room_id: host_jwt.access_token.room_id,
                details: Some(json!("Host cannot kick itself")),
            });
        }

        let mut room = self
            .rooms
            .get_mut(&host_jwt.access_token.room_id)
            .ok_or_else(|| RoomError::RoomNotFound {
                room_id: host_jwt.access_token.room_id,
            })?;

        let client = room
            .clients
            .remove(&client_id)
            .ok_or_else(|| RoomError::ClientNotFound {
                client_id,
                room_id: host_jwt.access_token.room_id,
            })?;

        for file_id in client.owned_files {
            let file_meta_maybe = room.files.remove(&file_id);
            if file_meta_maybe.is_none() {
                tracing::warn!(?file_id, room_id = ?host_jwt.access_token.room_id, "File not found")
            }
        }

        // TODO: broadcast updated room info
        // TODO: detect all clients that have files, not only one

        Ok(())
    }

    #[instrument(skip(self, jwt), err(Debug))]
    async fn announce_file(
        &self,
        jwt: JwtEncoded,
        file_meta: FileMeta,
    ) -> Result<FileId, RoomError> {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        self.check_jwt(&jwt)?;

        todo!()
    }

    #[instrument(skip(self, jwt), err(Debug))]
    async fn remove_file(&self, jwt: JwtEncoded, file_id: FileId) -> Result<(), RoomError> {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        self.check_jwt(&jwt)?;
        self.check_file_owner(&jwt, file_id)?;

        todo!()
    }

    #[instrument(skip(self, jwt), err(Debug))]
    async fn download(
        &self,
        jwt: JwtEncoded,
        file_id: FileId,
        offset_index: usize,
    ) -> Result<File, RoomError> {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        self.check_jwt(&jwt)?;

        todo!()
    }

    #[instrument(skip(self, jwt, subscription_sink), err(Debug))]
    async fn sub_info(
        &self,
        subscription_sink: jsonrpsee_core::server::PendingSubscriptionSink,
        jwt: JwtEncoded,
    ) -> jsonrpsee_core::SubscriptionResult {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        self.check_jwt(&jwt)?;

        todo!()
    }

    #[instrument(skip(self, jwt, subscription_sink), err(Debug))]
    async fn sub_download(
        &self,
        subscription_sink: jsonrpsee_core::server::PendingSubscriptionSink,
        jwt: JwtEncoded,
    ) -> jsonrpsee_core::SubscriptionResult {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        self.check_jwt(&jwt)?;

        todo!()
    }
}
