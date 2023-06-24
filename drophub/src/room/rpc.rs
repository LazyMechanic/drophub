#[cfg(feature = "rpc-server")]
use jsonrpsee::core::SubscriptionResult;
use jsonrpsee::proc_macros::rpc;

#[cfg(feature = "rpc-server")]
use crate::RoomError;
use crate::{
    AccessTokenEncoded, ClientId, EntityId, EntityMeta, Invite, InvitePassword, RoomEvent, RoomId,
    RoomOptions,
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
pub trait RoomRpc {
    /// Creates new invite.
    #[method(name = "invite")]
    fn invite(&self, tok: AccessTokenEncoded) -> Result<Invite, RoomError>;

    /// Revokes invite.
    #[method(name = "revoke_invite")]
    fn revoke_invite(
        &self,
        tok: AccessTokenEncoded,
        invite_password: InvitePassword,
    ) -> Result<(), RoomError>;

    /// Kicks client.
    #[method(name = "kick")]
    fn kick(&self, tok: AccessTokenEncoded, client_id: ClientId) -> Result<(), RoomError>;

    /// Announces new entity.
    #[method(name = "announce_entity")]
    fn announce_entity(
        &self,
        tok: AccessTokenEncoded,
        entity_meta: EntityMeta,
    ) -> Result<EntityId, RoomError>;

    /// Removes file.
    #[method(name = "remove_entity")]
    fn remove_entity(&self, tok: AccessTokenEncoded, entity_id: EntityId) -> Result<(), RoomError>;

    /// Creates new rpc and connect to it.
    #[subscription(name = "sub_create", unsubscribe = "unsub_create", item = RoomEvent)]
    async fn create(&self, options: RoomOptions) -> SubscriptionResult;

    /// Connects to existed rpc.
    #[subscription(name = "sub_connect", unsubscribe = "unsub_connect", item = RoomEvent)]
    async fn connect(&self, room_id: RoomId, invite_password: InvitePassword)
        -> SubscriptionResult;
}
