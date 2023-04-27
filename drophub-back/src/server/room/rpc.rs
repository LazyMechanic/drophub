// TODO: access to clients, files, downloads, etc. via failable funcs

use std::{ops::Add, pin::pin};

use anyhow::anyhow;
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use drophub::{
    ClientEvent, ClientId, DownloadProcId, FileData, FileId, FileMeta, Invite, InviteId,
    JwtEncoded, RoomError, RoomId, RoomOptions, RoomRpcServer,
};
use jsonrpsee::{
    core::{async_trait, SubscriptionResult},
    PendingSubscriptionSink,
};
use replace_with::replace_with_or_default;
use scopeguard::defer;
use time::OffsetDateTime;
use tokio::sync::{broadcast::error::RecvError, mpsc};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    config::{Config, RoomConfig},
    jwt::{AccessToken, ClientRole, Jwt, RefreshToken},
    server::room::{
        types::{Client, DownloadProc, File, Room},
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

    fn get_room(&self, room_id: RoomId) -> Result<Ref<'_, RoomId, Room>, RoomError> {
        self.rooms
            .get(&room_id)
            .ok_or(RoomError::RoomNotFound { room_id })
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

        let invite = room.generate_invite(self.cfg.invite_duration)?;
        room.invites
            .insert(invite.id.clone(), invite.clone(), self.cfg.invite_duration);

        // Broadcast updated room info
        let room_info = room.info();
        room.info_tx
            .send(room_info)
            .map_err(|_| anyhow!("Room info channel closed"))?;

        Ok(invite)
    }

    #[instrument(skip(self, jwt), err(Debug))]
    async fn revoke_invite(&self, jwt: JwtEncoded, invite_id: InviteId) -> Result<(), RoomError> {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        let mut room = self.get_room_mut(jwt.access_token.room_id)?;
        RoomValidator::new(&jwt, &*room).validate_revoke_invite()?;

        let _ = room
            .invites
            .remove(&invite_id)
            .ok_or(RoomError::InviteNotFound {
                invite_id,
                room_id: jwt.access_token.room_id,
            })?;

        // Broadcast updated room info
        let room_info = room.info();
        room.info_tx
            .send(room_info)
            .map_err(|_| anyhow!("Room info channel closed"))?;

        Ok(())
    }

    #[instrument(skip(self, jwt), err(Debug))]
    async fn kick(&self, jwt: JwtEncoded, client_id: ClientId) -> Result<(), RoomError> {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        let mut room = self.get_room_mut(jwt.access_token.room_id)?;
        RoomValidator::new(&jwt, &*room).validate_kick(client_id)?;

        // Remove client from list
        let _ = room
            .clients
            .remove(&client_id)
            .ok_or(RoomError::ClientNotFound {
                client_id,
                room_id: jwt.access_token.room_id,
            })?;

        // Remove all client-owned files
        // TODO: don't delete the file if any other client has it
        replace_with_or_default(&mut room.files, |room_files| {
            room_files
                .into_iter()
                .filter(|(_, file)| file.owner != client_id)
                .collect()
        });

        // Broadcast updated room info
        let room_info = room.info();
        room.info_tx
            .send(room_info)
            .map_err(|_| anyhow!("Room info channel closed"))?;

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
        RoomValidator::new(&jwt, &*room).validate_announce_file(file.id)?;

        let file_id = file.id;
        room.files.insert(file.id, file);

        // Broadcast updated room info
        let room_info = room.info();
        room.info_tx
            .send(room_info)
            .map_err(|_| anyhow!("Room info channel closed"))?;

        Ok(file_id)
    }

    #[instrument(skip(self, jwt), err(Debug))]
    async fn remove_file(&self, jwt: JwtEncoded, file_id: FileId) -> Result<(), RoomError> {
        let jwt = Jwt::decode(&jwt, &self.cfg.jwt.token_secret)?;
        let mut room = self.get_room_mut(jwt.access_token.room_id)?;
        RoomValidator::new(&jwt, &*room).validate_remove_file(file_id)?;

        room.files.remove(&file_id);

        // Broadcast updated room info
        let room_info = room.info();
        room.info_tx
            .send(room_info)
            .map_err(|_| anyhow!("Room info channel closed"))?;

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

        let mut download_proc =
            room.downloads
                .get_mut(&download_id)
                .ok_or(RoomError::DownloadProcessNotFound {
                    download_id,
                    room_id: jwt.access_token.room_id,
                })?;

        download_proc
            .data_tx
            .send(file_data)
            .await
            .map_err(|_| anyhow!("Data channel closed"))?;

        download_proc.next_block_idx += 1;
        if download_proc.next_block_idx >= download_proc.blocks_count {
            room.downloads.remove(&download_id);
        }

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
        let client_role = client.role;
        let client_id = client.id;
        let mut room = Room::new(options, self.cfg.block_size, client);

        let host_jwt = Jwt {
            access_token: AccessToken {
                client_id,
                room_id: room.id,
                role: client_role,
                exp: self
                    .cfg
                    .jwt
                    .access_token_duration
                    .clone()
                    .map(|dur| OffsetDateTime::now_utc().add(dur)),
            },
            refresh_token: RefreshToken {
                token: Uuid::new_v4(),
                exp: self
                    .cfg
                    .jwt
                    .refresh_token_duration
                    .clone()
                    .map(|dur| OffsetDateTime::now_utc().add(dur)),
            },
        }
        .encode(&self.cfg.jwt.token_secret)?;

        let sink = subscription_sink.accept().await?;
        let mut sink_closed = pin!(sink.closed());
        let mut room_rx = room.info_tx.subscribe();

        sink.send(ClientEvent::Init(host_jwt).try_into()?).await?;

        // Broadcast updated room info
        let room_info = room.info();
        room.info_tx
            .send(room_info)
            .map_err(|_| anyhow!("Room info channel closed"))?;

        let room_id = room.id;
        self.rooms.insert(room_id, room);
        defer! { self.rooms.remove(&room_id); }

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
        let client_id = client.id;

        let guest_jwt = Jwt {
            access_token: AccessToken {
                client_id,
                room_id,
                role: client.role,
                exp: self
                    .cfg
                    .jwt
                    .access_token_duration
                    .clone()
                    .map(|dur| OffsetDateTime::now_utc().add(dur)),
            },
            refresh_token: RefreshToken {
                token: Uuid::new_v4(),
                exp: self
                    .cfg
                    .jwt
                    .refresh_token_duration
                    .clone()
                    .map(|dur| OffsetDateTime::now_utc().add(dur)),
            },
        }
        .encode(&self.cfg.jwt.token_secret)?;

        let mut room_rx = {
            let mut room = self
                .rooms
                .get_mut(&room_id)
                .ok_or(RoomError::RoomNotFound { room_id })?;

            let _ = room
                .invites
                .remove(&invite_id)
                .ok_or(RoomError::InviteNotFound { invite_id, room_id })?;

            // Add connected client
            room.clients.insert(client.id, client);

            let room_rx = room.info_tx.subscribe();

            // Broadcast updated room info
            let room_info = room.info();
            room.info_tx
                .send(room_info)
                .map_err(|_| anyhow!("Room info channel closed"))?;

            room_rx
        };

        // Remove client from room on disconnect
        defer! {
            let Some(mut room) = self.rooms.get_mut(&room_id) else { return };
            // Remove client from list
            room.clients.remove(&client_id);

            // Remove all client-owned files
            // TODO: don't delete the file if any other client has it
            replace_with_or_default(&mut room.files, |room_files| {
                room_files
                    .into_iter()
                    .filter(|(_, file)| file.owner != client_id)
                    .collect()
            });
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

            let file = room.files.get(&file_id).ok_or(RoomError::FileNotFound {
                file_id,
                room_id: jwt.access_token.room_id,
            })?;

            let (data_tx, data_rx) = mpsc::channel(1);
            let download_proc =
                DownloadProc::new(file_id, file.meta.size, self.cfg.block_size, data_tx);

            let download_proc_id = download_proc.id;
            room.downloads.insert(download_proc.id, download_proc);

            (download_proc_id, data_rx)
        };

        defer! {
            let Some(mut room) = self.rooms.get_mut(&jwt.access_token.room_id) else { return };
            room.downloads.remove(&download_proc_id);
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
