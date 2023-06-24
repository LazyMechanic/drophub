// TODO: add encryption

use std::{ops::Add, pin::pin};

use dashmap::{mapref::one::RefMut, DashMap};
use drophub::{
    AccessToken, AccessTokenEncoded, ClientId, ClientRole, EntityId, EntityMeta, Invite,
    InvitePassword, RoomError, RoomEvent, RoomId, RoomOptions, RoomRpcServer,
};
use jsonrpsee::{
    core::{async_trait, SubscriptionResult},
    PendingSubscriptionSink,
};
use scopeguard::defer;
use time::OffsetDateTime;
use tokio::sync::broadcast::error::RecvError;
use tracing::{instrument, Instrument};

use crate::{
    config::{Config, RoomConfig},
    server::room::{
        types::{Client, Entity, Room},
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
    #[instrument(skip(self, tok), err(Debug))]
    fn invite(&self, tok: AccessTokenEncoded) -> Result<Invite, RoomError> {
        let tok = AccessToken::decode(&tok, &self.cfg.jwt.secret)?;
        let _span = tok.create_span().entered();

        let mut room = self.get_room_mut(tok.room_id)?;
        RoomValidator::new(&tok, &mut *room).validate_invite()?;

        let invite = room.generate_invite()?;
        tracing::info!(?invite, "Invite created");
        room.broadcast_info()?;

        Ok(invite)
    }

    #[instrument(skip(self, tok), err(Debug))]
    fn revoke_invite(
        &self,
        tok: AccessTokenEncoded,
        invite_id: InvitePassword,
    ) -> Result<(), RoomError> {
        let tok = AccessToken::decode(&tok, &self.cfg.jwt.secret)?;
        let _span = tok.create_span().entered();

        let mut room = self.get_room_mut(tok.room_id)?;
        RoomValidator::new(&tok, &*room).validate_revoke_invite()?;

        room.revoke_invite(invite_id)?;
        room.broadcast_info()?;

        Ok(())
    }

    #[instrument(skip(self, tok), err(Debug))]
    fn kick(&self, tok: AccessTokenEncoded, client_id: ClientId) -> Result<(), RoomError> {
        let tok = AccessToken::decode(&tok, &self.cfg.jwt.secret)?;
        let _span = tok.create_span().entered();

        let mut room = self.get_room_mut(tok.room_id)?;
        RoomValidator::new(&tok, &*room).validate_kick(client_id)?;

        room.remove_client(client_id)?;
        room.broadcast_info()?;

        Ok(())
    }

    #[instrument(skip(self, tok), err(Debug))]
    fn announce_entity(
        &self,
        tok: AccessTokenEncoded,
        file_meta: EntityMeta,
    ) -> Result<EntityId, RoomError> {
        let tok = AccessToken::decode(&tok, &self.cfg.jwt.secret)?;
        let _span = tok.create_span().entered();

        let mut room = self.get_room_mut(tok.room_id)?;
        let entity = Entity::new(tok.client_id, file_meta);
        let entity_id = entity.id();
        RoomValidator::new(&tok, &*room).validate_announce_entity(entity_id)?;

        room.add_entity(entity)?;
        room.broadcast_info()?;

        Ok(entity_id)
    }

    #[instrument(skip(self, tok), err(Debug))]
    fn remove_entity(&self, tok: AccessTokenEncoded, entity_id: EntityId) -> Result<(), RoomError> {
        let tok = AccessToken::decode(&tok, &self.cfg.jwt.secret)?;
        let _span = tok.create_span().entered();

        let mut room = self.get_room_mut(tok.room_id)?;
        RoomValidator::new(&tok, &*room).validate_remove_entity(entity_id)?;

        room.remove_entity(entity_id)?;
        room.broadcast_info()?;

        Ok(())
    }

    #[instrument(skip(self, subscription_sink), err(Debug))]
    async fn create(
        &self,
        subscription_sink: PendingSubscriptionSink,
        options: RoomOptions,
    ) -> SubscriptionResult {
        // TODO: switch roles with a random client on disconnect

        let client = Client::new(ClientRole::Host);
        let client_role = client.role();
        let client_id = client.id();
        let mut room = Room::new(options, client, &self.cfg);

        let host_token = AccessToken {
            client_id,
            room_id: room.id(),
            role: client_role,
            exp: self
                .cfg
                .jwt
                .ttl
                .map(|dur| OffsetDateTime::now_utc().add(dur)),
        };
        let span = host_token.create_span();

        async move {
            let host_token = host_token.encode(&self.cfg.jwt.secret)?;

            let mut room_rx = {
                let room_rx = room.subscribe();
                room.broadcast_info()?;
                room_rx
            };

            let room_id = room.id();
            let room_host_id = room.host_id();
            self.rooms.insert(room_id, room);
            tracing::info!(?room_id, ?room_host_id, "Room created");

            // Remove room on disconnect
            defer! {
                self.rooms.remove(&room_id);
                tracing::info!(?room_id, "Room removed");
            }

            let sink = subscription_sink.accept().await?;
            let mut sink_closed = pin!(sink.closed());

            sink.send(
                RoomEvent::Init {
                    token: host_token,
                    client_id,
                    client_role,
                }
                .try_into()?,
            )
            .await?;

            loop {
                tokio::select! {
                    _ = &mut sink_closed => {
                        tracing::debug!("Subscription sink closed");
                        break Ok(());
                    }
                    maybe_room_info = room_rx.recv() => match maybe_room_info {
                        Ok(room_info) => {
                            tracing::debug!(?room_info, "Received room info");
                            sink.send(RoomEvent::RoomInfo(room_info).try_into()?).await?;
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
                }
            }
        }
        .instrument(span)
        .await
    }

    #[instrument(skip(self, subscription_sink), err(Debug))]
    async fn connect(
        &self,
        subscription_sink: PendingSubscriptionSink,
        room_id: RoomId,
        invite_password: InvitePassword,
    ) -> SubscriptionResult {
        let client = Client::new(ClientRole::Guest);
        let client_id = client.id();
        let client_role = client.role();

        let guest_token = AccessToken {
            client_id,
            room_id,
            role: client.role(),
            exp: self
                .cfg
                .jwt
                .ttl
                .map(|dur| OffsetDateTime::now_utc().add(dur)),
        };
        let span = guest_token.create_span();

        async move {
            let guest_token = guest_token.encode(&self.cfg.jwt.secret)?;
            //panic!("{}", guest_token);

            let mut room_rx = {
                let mut room = self.get_room_mut(room_id)?;
                let room_rx = room.add_client(client, invite_password)?;
                tracing::info!(?client_id, "Client connected to room");
                room.broadcast_info()?;

                room_rx
            };

            // Remove client from room on disconnect
            defer! {
                let Some(mut room) = self.rooms.get_mut(&room_id) else { return };
                let _ = room.remove_client(client_id);
                tracing::info!(?client_id, "Client disconnected");
            }

            let sink = subscription_sink.accept().await?;
            let mut sink_closed = pin!(sink.closed());

            sink.send(
                RoomEvent::Init {
                    token: guest_token,
                    client_id,
                    client_role,
                }
                .try_into()?,
            )
            .await?;

            loop {
                tokio::select! {
                    _ = &mut sink_closed => {
                        tracing::debug!("Subscription sink closed");
                        break Ok(());
                    }
                    maybe_room_info = room_rx.recv() => match maybe_room_info {
                        Ok(room_info) => {
                            tracing::debug!(?room_info, "Received room info");
                            sink.send(RoomEvent::RoomInfo(room_info).try_into()?).await?;
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
                }
            }
        }
        .instrument(span)
        .await
    }
}
