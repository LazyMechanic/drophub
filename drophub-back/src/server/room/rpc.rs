// TODO: access to clients, files, downloads, etc. via failable funcs

use std::{ops::Add, pin::pin};

use dashmap::{mapref::one::RefMut, DashMap};
use drophub::{
    ClientEvent, ClientId, DownloadProcId, FileData, FileId, FileMeta, Invite, InviteId,
    JwtEncoded, RoomError, RoomId, RoomOptions, RoomRpcServer,
};
use jsonrpsee::{
    core::{async_trait, SubscriptionResult},
    PendingSubscriptionSink,
};
use scopeguard::defer;
use time::OffsetDateTime;
use tokio::sync::{broadcast::error::RecvError, mpsc};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    config::{Config, RoomConfig},
    jwt::{AccessToken, ClientRole, Jwt, RefreshToken},
    server::room::{
        types::{Client, File, Room},
        validator::{RoomMutValidate, RoomValidate, RoomValidator},
    },
};

#[derive(Debug)]
pub struct RoomRpc {
    rooms: DashMap<RoomId, Room>,
    cfg: RoomConfig,
}

impl RoomRpc {
    pub fn new(cfg: &Config) -> Self {
        Self {
            rooms: Default::default(),
            cfg: cfg.room.clone(),
        }
    }

    fn get_room_mut(&self, room_id: RoomId) -> Result<RefMut<'_, RoomId, Room>, RoomError> {
        self.rooms
            .get_mut(&room_id)
            .ok_or(RoomError::RoomNotFound { room_id })
    }
}

#[async_trait]
impl RoomRpcServer for RoomRpc {
    #[instrument(skip(self, jwt), err(Debug))]
    async fn invite(&self, jwt: JwtEncoded) -> Result<Invite, RoomError> {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        let mut room = self.get_room_mut(jwt.access_token.room_id)?;
        RoomValidator::new(&jwt, &mut *room).validate_invite()?;

        let invite = room.generate_invite()?;
        room.broadcast_info()?;

        Ok(invite)
    }

    #[instrument(skip(self, jwt), err(Debug))]
    async fn revoke_invite(&self, jwt: JwtEncoded, invite_id: InviteId) -> Result<(), RoomError> {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        let mut room = self.get_room_mut(jwt.access_token.room_id)?;
        RoomValidator::new(&jwt, &*room).validate_revoke_invite()?;

        room.revoke_invite(invite_id)?;
        room.broadcast_info()?;

        Ok(())
    }

    #[instrument(skip(self, jwt), err(Debug))]
    async fn kick(&self, jwt: JwtEncoded, client_id: ClientId) -> Result<(), RoomError> {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        let mut room = self.get_room_mut(jwt.access_token.room_id)?;
        RoomValidator::new(&jwt, &*room).validate_kick(client_id)?;

        room.remove_client(client_id)?;
        room.broadcast_info()?;

        Ok(())
    }

    #[instrument(skip(self, jwt), err(Debug))]
    async fn announce_file(
        &self,
        jwt: JwtEncoded,
        file_meta: FileMeta,
    ) -> Result<FileId, RoomError> {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        let mut room = self.get_room_mut(jwt.access_token.room_id)?;
        let file = File::new(file_meta, jwt.access_token.client_id);
        let file_id = file.id();
        RoomValidator::new(&jwt, &*room).validate_announce_file(file_id)?;

        room.add_file(file)?;
        room.broadcast_info()?;

        Ok(file_id)
    }

    #[instrument(skip(self, jwt), err(Debug))]
    async fn remove_file(&self, jwt: JwtEncoded, file_id: FileId) -> Result<(), RoomError> {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        let mut room = self.get_room_mut(jwt.access_token.room_id)?;
        RoomValidator::new(&jwt, &*room).validate_remove_file(file_id)?;

        room.remove_file(file_id)?;
        room.broadcast_info()?;

        Ok(())
    }

    #[instrument(skip(self, jwt, file_data), err(Debug))]
    async fn upload_file(
        &self,
        jwt: JwtEncoded,
        file_data: FileData,
        download_id: DownloadProcId,
    ) -> Result<(), RoomError> {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        let mut room = self.get_room_mut(jwt.access_token.room_id)?;
        RoomValidator::new(&jwt, &*room).validate_upload_file()?;

        room.send_file(file_data, download_id).await?;

        Ok(())
    }

    #[instrument(skip(self, subscription_sink), err(Debug))]
    async fn create(
        &self,
        subscription_sink: PendingSubscriptionSink,
        options: RoomOptions,
    ) -> SubscriptionResult {
        // TODO: switch roles with a random client on disconnect

        let (upload_tx, mut upload_rx) = mpsc::unbounded_channel();
        let client = Client::new(ClientRole::Host, upload_tx);
        let client_role = client.role();
        let client_id = client.id();
        let mut room = Room::new(options, client, &self.cfg);

        let host_jwt = Jwt {
            access_token: AccessToken {
                client_id,
                room_id: room.id(),
                role: client_role,
                exp: self
                    .cfg
                    .jwt
                    .access_token_ttl
                    .clone()
                    .map(|dur| OffsetDateTime::now_utc().add(dur)),
            },
            refresh_token: RefreshToken {
                token: Uuid::new_v4(),
                exp: self
                    .cfg
                    .jwt
                    .refresh_token_ttl
                    .clone()
                    .map(|dur| OffsetDateTime::now_utc().add(dur)),
            },
        }
        .encode(&self.cfg.jwt.token_secret)?;

        let mut room_rx = {
            let room_rx = room.subscribe();
            room.broadcast_info()?;
            room_rx
        };

        let room_id = room.id();
        self.rooms.insert(room_id, room);
        // Remove room on disconnect
        defer! { self.rooms.remove(&room_id); }

        let sink = subscription_sink.accept().await?;
        let mut sink_closed = pin!(sink.closed());

        sink.send(ClientEvent::Init(host_jwt).try_into()?).await?;

        loop {
            tokio::select! {
                _ = &mut sink_closed => {
                    tracing::debug!("Subscription sink closed");
                    break Ok(());
                }
                maybe_room_info = room_rx.recv() => match maybe_room_info {
                    Ok(room_info) => {
                        tracing::debug!(?room_info, "Received room info");
                        sink.send(ClientEvent::RoomInfo(room_info).try_into()?).await?;
                    }
                    Err(err @ RecvError::Lagged(_)) => {
                        tracing::warn!(?err, "Received lag");
                        continue;
                    }
                    Err(RecvError::Closed) => {
                        tracing::debug!("Room closed (room info channel closed)");
                        break Ok(());
                    }
                },
                maybe_upload_req = upload_rx.recv() => match maybe_upload_req {
                    Some(upload_req) => {
                        tracing::debug!(?upload_req, "Received upload request");
                        sink.send(ClientEvent::UploadRequest(upload_req).try_into()?).await?;
                    }
                    None => {
                        tracing::debug!("Client kicked (upload request channel closed)");
                        break Ok(());
                    }
                },
            }
        }
    }

    #[instrument(skip(self, subscription_sink), err(Debug))]
    async fn connect(
        &self,
        subscription_sink: PendingSubscriptionSink,
        room_id: RoomId,
        invite_id: InviteId,
    ) -> SubscriptionResult {
        let (upload_tx, mut upload_rx) = mpsc::unbounded_channel();
        let client = Client::new(ClientRole::Guest, upload_tx);
        let client_id = client.id();

        let guest_jwt = Jwt {
            access_token: AccessToken {
                client_id,
                room_id,
                role: client.role(),
                exp: self
                    .cfg
                    .jwt
                    .access_token_ttl
                    .clone()
                    .map(|dur| OffsetDateTime::now_utc().add(dur)),
            },
            refresh_token: RefreshToken {
                token: Uuid::new_v4(),
                exp: self
                    .cfg
                    .jwt
                    .refresh_token_ttl
                    .clone()
                    .map(|dur| OffsetDateTime::now_utc().add(dur)),
            },
        }
        .encode(&self.cfg.jwt.token_secret)?;

        let mut room_rx = {
            let mut room = self.get_room_mut(room_id)?;
            let room_rx = room.add_client(client, invite_id)?;
            room.broadcast_info()?;

            room_rx
        };

        // Remove client from room on disconnect
        defer! {
            let Some(mut room) = self.rooms.get_mut(&room_id) else { return };
            let _ = room.remove_client(client_id);
        }

        let sink = subscription_sink.accept().await?;
        let mut sink_closed = pin!(sink.closed());

        sink.send(ClientEvent::Init(guest_jwt).try_into()?).await?;

        loop {
            tokio::select! {
                _ = &mut sink_closed => {
                    tracing::debug!("Subscription sink closed");
                    break Ok(());
                }
                maybe_room_info = room_rx.recv() => match maybe_room_info {
                    Ok(room_info) => {
                        tracing::debug!(?room_info, "Received room info");
                        sink.send(ClientEvent::RoomInfo(room_info).try_into()?).await?;
                    }
                    Err(err @ RecvError::Lagged(_)) => {
                        tracing::warn!(?err, "Received lag");
                        continue;
                    }
                    Err(RecvError::Closed) => {
                        tracing::debug!("Room closed (room info channel closed)");
                        break Ok(());
                    }
                },
                maybe_upload_req = upload_rx.recv() => match maybe_upload_req {
                    Some(upload_req) => {
                        tracing::debug!(?upload_req, "Received upload request");
                        sink.send(ClientEvent::UploadRequest(upload_req).try_into()?).await?;
                    }
                    None => {
                        tracing::debug!("Client kicked (upload request channel closed)");
                        break Ok(());
                    }
                },
            }
        }
    }

    #[instrument(skip(self, jwt, subscription_sink), err(Debug))]
    async fn sub_download(
        &self,
        subscription_sink: PendingSubscriptionSink,
        jwt: JwtEncoded,
        file_id: FileId,
    ) -> SubscriptionResult {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        let (download_proc_id, mut data_rx) = {
            let mut room = self.get_room_mut(jwt.access_token.room_id)?;
            RoomValidator::new(&jwt, &*room).validate_sub_download(file_id)?;
            room.start_download_proc(file_id)?
        };

        defer! {
            let Some(mut room) = self.rooms.get_mut(&jwt.access_token.room_id) else { return };
            room.stop_download_proc(download_proc_id)
        }

        let sink = subscription_sink.accept().await?;
        let mut sink_closed = pin!(sink.closed());

        loop {
            tokio::select! {
                _ = &mut sink_closed => {
                    tracing::debug!("Subscription sink closed");
                    break Ok(());
                }
                maybe_data = data_rx.recv() => match maybe_data {
                    Some(data) => {
                        tracing::debug!("Received file data");
                        sink.send(data.try_into()?).await?;
                    }
                    None => {
                        tracing::debug!("File downloading done");
                        break Ok(());
                    }
                }
            }
        }
    }
}
