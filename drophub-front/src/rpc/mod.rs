mod channel;

use drophub::{ClientEvent, RoomRpcClient};
use jsonrpsee::core::client::Subscription;
use yew::platform::spawn_local;

pub use self::channel::{
    channel, RpcRequestTx, RpcSingleRequest, RpcSingleResponse, RpcSubscribeRequest,
    RpcSubscribeResponse, RpcSubscribeResponseRx,
};
use crate::{
    config::Config,
    error::Error,
    rpc::channel::{RpcRequestMsg, RpcRequestRx, RpcSubscribeResponseTx},
};

pub async fn run(cfg: Config, mut rpc_rx: RpcRequestRx) {
    let rpc_client = jsonrpsee::wasm_client::WasmClientBuilder::default()
        .build(cfg.api_root_url)
        .await
        .unwrap();

    while let Some(req_msg) = rpc_rx.recv().await {
        if let Err(err) = handle_req_msg(&rpc_client, req_msg).await {
            // TODO: show error
        }
    }
}

async fn handle_req_msg(
    rpc_client: &jsonrpsee::core::client::Client,
    req_msg: RpcRequestMsg,
) -> Result<(), Error> {
    match req_msg {
        RpcRequestMsg::Single(req, resp_tx) => match req {},
        RpcRequestMsg::Subscribe(req, resp_tx) => match req {
            RpcSubscribeRequest::CreateRoom(opt) => {
                let sub = rpc_client.create(opt).await?;
                spawn_local(create_room(sub, resp_tx));
            }
        },
    }

    Ok(())
}

async fn create_room(sub: Subscription<ClientEvent>, resp_tx: RpcSubscribeResponseTx) {}
