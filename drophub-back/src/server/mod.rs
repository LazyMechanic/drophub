mod rpc;
mod storage;
#[cfg(test)]
mod tests;

use std::net::SocketAddr;

use drophub::RpcServer;
use jsonrpsee::server::{ServerBuilder, ServerHandle};

use self::rpc::Rpc;
use crate::config::Config;

pub async fn run(cfg: Config) -> anyhow::Result<(SocketAddr, ServerHandle)> {
    let server = ServerBuilder::default()
        .ws_only()
        .build(cfg.server.bind_addr)
        .await?;

    let rpc = Rpc::new(cfg).await?;

    let addr = server.local_addr()?;
    let handle = server.start(rpc.into_rpc())?;
    tracing::info!(?addr, "Server started");

    Ok((addr, handle))
}
