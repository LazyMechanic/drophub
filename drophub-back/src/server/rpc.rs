use std::pin::pin;

use chrono::{Duration, Utc};
use drophub::{
    AnnouncedEntity, Entity, EntityId, Error, InvitePassphrase, PeerEvent, PeerId, PeerToken,
    PeerTokenEncoded, Room, RoomId, RpcServer,
};
use futures::FutureExt;
use jsonrpsee::{
    core::{async_trait, SubscriptionResult},
    PendingSubscriptionSink,
};
use mongodb::options::ClientOptions;
use rand::Rng;
use scopeguard::defer;
use tracing::{instrument, Instrument};
use uuid::Uuid;

use super::storage;
use crate::config::Config;

pub struct Rpc {
    mongodb_client: mongodb::Client,
    cfg: Config,
}

impl Rpc {
    pub async fn new(cfg: Config) -> anyhow::Result<Self> {
        let mut client_options = ClientOptions::parse(&cfg.mongodb.uri).await?;
        client_options.app_name = Some(env!("CARGO_PKG_NAME").to_owned());

        let mongodb_client = mongodb::Client::with_options(client_options)?;

        Ok(Self {
            mongodb_client,
            cfg,
        })
    }
}

#[async_trait]
impl RpcServer for Rpc {
    #[instrument(skip(self))]
    fn invite(
        &self,
        token: PeerTokenEncoded,
        invite_passphrase: InvitePassphrase,
    ) -> Result<(), Error> {
        todo!()
    }

    fn announce_entity(
        &self,
        token: PeerTokenEncoded,
        entity: AnnouncedEntity,
    ) -> Result<EntityId, Error> {
        todo!()
    }

    fn remove_entity(&self, token: PeerTokenEncoded, entity_id: EntityId) -> Result<(), Error> {
        todo!()
    }

    fn get_room_state(&self, token: PeerTokenEncoded) -> Result<Room, Error> {
        todo!()
    }

    async fn sub_peer_events(
        &self,
        subscription_sink: PendingSubscriptionSink,
    ) -> SubscriptionResult {
        let sink = subscription_sink.accept().await?;
        let mut subscribe_closed = pin!(sink.closed());

        let peer_id = Uuid::new_v4();
        let init_token = PeerToken {
            peer_id,
            room_id: None,
            exp: None,
        };
        let init_token = init_token.encode(&self.cfg.server.secret)?;
        let invite_passphrase = create_invite(&self.mongodb_client, peer_id).await?;

        defer! {
            let mongodb_client = self.mongodb_client.clone();
            tokio::spawn(async move {
                let _ = storage::remove_invite(&mongodb_client, &invite_passphrase).await;
            });
        }

        sink.send(
            PeerEvent::Init {
                token: init_token,
                invite_passphrase,
            }
            .try_into()?,
        )
        .await?;

        loop {
            tokio::select! {

                _ = &mut subscribe_closed => {
                    tracing::info!("Subscription closed");
                    return Ok(())
                }
            }
        }
    }
}

async fn create_invite(
    mongodb_client: &mongodb::Client,
    peer_id: PeerId,
) -> Result<InvitePassphrase, Error> {
    let mut session = mongodb_client.start_session(None).await?;

    let invite_passphrase = session
        .with_transaction(
            (),
            |session, _ctx| {
                async move {
                    let client = session.client();
                    // While not found free unique alias
                    loop {
                        let invite_passphrase = generate_invite_passphrase();

                        match storage::get_invite(&client, &invite_passphrase).await? {
                            None => {
                                storage::add_invite(
                                    &client,
                                    storage::Invite {
                                        passphrase: invite_passphrase.clone(),
                                        peer_id,
                                        create_at: Utc::now(),
                                    },
                                )
                                .await?;
                            }
                            Some(invite) if invite.is_expired() => {
                                storage::remove_invite(&client, &invite_passphrase).await?;
                                storage::add_invite(
                                    &client,
                                    storage::Invite {
                                        passphrase: invite_passphrase.clone(),
                                        peer_id,
                                        create_at: Utc::now(),
                                    },
                                )
                                .await?;
                            }
                            _ => continue,
                        }

                        return Ok(invite_passphrase);
                    }
                }
                .boxed()
            },
            None,
        )
        .await?;

    Ok(invite_passphrase)
}

fn generate_invite_passphrase() -> String {
    const LEN: usize = 6;
    const CHARSET: [char; 31] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k',
        'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut rng = rand::thread_rng();
    (0..LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx]
        })
        .collect()
}
