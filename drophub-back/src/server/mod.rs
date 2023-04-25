mod room;

use drophub::RoomRpcServer;
use jsonrpsee::server::ServerBuilder;

use crate::{config::Config, server::room::RoomRpc};

pub async fn run(cfg: &Config) -> anyhow::Result<()> {
    let server = ServerBuilder::default()
        .ws_only()
        .build(cfg.server.bind_addr)
        .await?;

    let room = RoomRpc::new(cfg);

    let addr = server.local_addr()?;
    let handle = server.start(room.into_rpc())?;
    tracing::info!(?addr, "Server started");
    handle.stopped().await;

    Ok(())
}
