use jsonrpsee::{core::SubscriptionResult, proc_macros::rpc};

use crate::{
    ClientId, File, FileId, FileMeta, Invite, InviteId, JwtEncoded, RoomError, RoomId, RoomInfo,
    RoomOptions,
};

#[rpc(client, server, namespace = "room")]
pub trait RoomRpc {
    #[method(name = "create")]
    async fn create(&self, options: RoomOptions) -> Result<JwtEncoded, RoomError>;

    #[method(name = "connect")]
    async fn connect(&self, room_id: RoomId, invite_id: InviteId) -> Result<JwtEncoded, RoomError>;

    #[method(name = "invite")]
    async fn invite(&self, host_jwt: JwtEncoded) -> Result<Invite, RoomError>;

    #[method(name = "revoke_invite")]
    async fn revoke_invite(
        &self,
        host_jwt: JwtEncoded,
        invite_id: InviteId,
    ) -> Result<(), RoomError>;

    #[method(name = "kick")]
    async fn kick(&self, host_jwt: JwtEncoded, client_id: ClientId) -> Result<(), RoomError>;

    #[method(name = "announce_file")]
    async fn announce_file(
        &self,
        jwt: JwtEncoded,
        file_meta: FileMeta,
    ) -> Result<FileId, RoomError>;

    #[method(name = "remove_file")]
    async fn remove_file(&self, jwt: JwtEncoded, file_id: FileId) -> Result<(), RoomError>;

    #[method(name = "download")]
    async fn download(
        &self,
        jwt: JwtEncoded,
        file_id: FileId,
        offset_index: usize,
    ) -> Result<File, RoomError>;

    #[subscription(name = "sub_info", unsubscribe = "unsub_info", item = RoomInfo)]
    async fn sub_info(&self, jwt: JwtEncoded) -> SubscriptionResult;

    #[subscription(name = "sub_download", unsubscribe = "unsub_download", item = FileId)]
    async fn sub_download(&self, jwt: JwtEncoded) -> SubscriptionResult;
}
