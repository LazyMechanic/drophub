pub mod channel;

pub use channel::{channel, RpcRequest, RpcRequestReceiver, RpcRequestSender, RpcResponse};
use drophub::RoomOptions;
use yew::platform::spawn_local;

use crate::{config::Config, rpc::channel::RoomRpcModule};

pub async fn run(cfg: Config, mut rpc_rx: RpcRequestReceiver) {
    let rpc_client = jsonrpsee::wasm_client::WasmClientBuilder::default()
        .build(cfg.api_root_url)
        .await
        .unwrap();

    while let Some((req, resp_tx)) = rpc_rx.recv() {
        match req {
            RpcRequest::Room(RoomRpcModule::Create(opt)) => spawn_local(create(opt)),
        }
    }
}

async fn create(opt: RoomOptions) {
    todo!()
}
