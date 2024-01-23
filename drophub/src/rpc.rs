#[cfg(feature = "rpc-server")]
use jsonrpsee::core::SubscriptionResult;
use jsonrpsee::proc_macros::rpc;

#[cfg(feature = "rpc-server")]
use crate::Error;
use crate::{
    AnnouncedEntity, EntityId, InvitePassphrase, PeerEvent, PeerId, PeerTokenEncoded, Room,
};

#[cfg_attr(
    all(
        any(feature = "rpc-client-ws", feature = "rpc-client-wasm"),
        not(feature = "rpc-server")
    ),
    rpc(client, namespace = "rpc")
)]
#[cfg_attr(
    all(
        feature = "rpc-server",
        not(any(feature = "rpc-client-ws", feature = "rpc-client-wasm"))
    ),
    rpc(server, namespace = "rpc")
)]
#[cfg_attr(
    all(
        any(feature = "rpc-client-ws", feature = "rpc-client-wasm"),
        feature = "rpc-server"
    ),
    rpc(client, server, namespace = "rpc")
)]
pub trait Rpc {
    /// Invite peer to room.
    #[method(name = "invite")]
    fn invite(
        &self,
        token: PeerTokenEncoded,
        invite_passphrase: InvitePassphrase,
    ) -> Result<(), Error>;

    /// Announces new entity.
    #[method(name = "announce_entity")]
    fn announce_entity(
        &self,
        token: PeerTokenEncoded,
        entity: AnnouncedEntity,
    ) -> Result<EntityId, Error>;

    /// Removes file.
    #[method(name = "remove_entity")]
    fn remove_entity(&self, token: PeerTokenEncoded, entity_id: EntityId) -> Result<(), Error>;

    /// Get current room state.
    #[method(name = "get_room_state")]
    fn get_room_state(&self, token: PeerTokenEncoded) -> Result<Room, Error>;

    /// Subscribe to invitation.
    #[subscription(name = "sub_peer_events", unsubscribe = "unsub_peer_events", item = PeerEvent)]
    async fn sub_peer_events(&self) -> SubscriptionResult;
}
