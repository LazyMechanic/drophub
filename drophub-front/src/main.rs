mod app;
mod config;
mod rpc;

use app::App;
use drophub::RoomRpcClient;
use yew::platform::{pinned::mpsc, spawn_local};

use crate::{
    config::Config,
    rpc_msg::{rpc_channel, RpcRequest, RpcRequestReceiver},
};

fn main() -> anyhow::Result<()> {
    init_logging();
    run_client()
}

fn init_logging() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
}

fn run_client() -> anyhow::Result<()> {
    let cfg = Config::from_env()?;
    let (rpc_tx, rpc_rx) = rpc::channel();

    // TODO: pass rpc_tx to app
    spawn_local(rpc::run(cfg.clone(), rpc_rx));
    yew::Renderer::<App>::with_props(cfg).render();

    Ok(())
}
