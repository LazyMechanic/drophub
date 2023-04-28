#[cfg(feature = "rpc-server")]
use jsonrpsee::core::SubscriptionResult;
use jsonrpsee::proc_macros::rpc;

#[cfg(feature = "rpc-server")]
use crate::RoomError;
use crate::{
    ClientEvent, ClientId, DownloadProcId, FileData, FileId, FileMeta, Invite, InviteId,
    JwtEncoded, RoomId, RoomOptions,
};

#[cfg_attr(
    all(
        any(feature = "rpc-client-ws", feature = "rpc-client-wasm"),
        not(feature = "rpc-server")
    ),
    rpc(client, namespace = "room")
)]
#[cfg_attr(
    all(
        feature = "rpc-server",
        not(any(feature = "rpc-client-ws", feature = "rpc-client-wasm"))
    ),
    rpc(server, namespace = "room")
)]
#[cfg_attr(
    all(
        any(feature = "rpc-client-ws", feature = "rpc-client-wasm"),
        feature = "rpc-server"
    ),
    rpc(client, server, namespace = "room")
)]
pub trait RoomRpc {
    /// Creates new invite.
    #[method(name = "invite")]
    fn invite(&self, jwt: JwtEncoded) -> Result<Invite, RoomError>;

    /// Revokes invite.
    #[method(name = "revoke_invite")]
    fn revoke_invite(&self, jwt: JwtEncoded, invite_id: InviteId) -> Result<(), RoomError>;

    /// Kicks client.
    #[method(name = "kick")]
    fn kick(&self, jwt: JwtEncoded, client_id: ClientId) -> Result<(), RoomError>;

    /// Announces new file.
    #[method(name = "announce_file")]
    fn announce_file(&self, jwt: JwtEncoded, file_meta: FileMeta) -> Result<FileId, RoomError>;

    /// Removes file.
    #[method(name = "remove_file")]
    fn remove_file(&self, jwt: JwtEncoded, file_id: FileId) -> Result<(), RoomError>;

    /// Uploads received file.
    #[method(name = "upload_file")]
    async fn upload_file(
        &self,
        jwt: JwtEncoded,
        file_data: FileData,
        download_id: DownloadProcId,
    ) -> Result<(), RoomError>;

    /// Creates new room and connect to it.
    #[subscription(name = "sub_create", unsubscribe = "unsub_create", item = ClientEvent)]
    async fn create(&self, options: RoomOptions) -> SubscriptionResult;

    /// Connects to existed room.
    #[subscription(name = "sub_connect", unsubscribe = "unsub_connect", item = ClientEvent)]
    async fn connect(&self, room_id: RoomId, invite_id: InviteId) -> SubscriptionResult;

    /// Subscribes to receive the file block by block.
    #[subscription(name = "sub_download", unsubscribe = "unsub_download", item = FileData)]
    async fn sub_download(&self, jwt: JwtEncoded, file_id: FileId) -> SubscriptionResult;
}
