use std::rc::Rc;

use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yewdux::prelude::*;

#[hook]
pub fn use_rpc_storage() -> (Rc<RpcStorage>, Dispatch<RpcStorage>) {
    use_store::<RpcStorage>()
}

#[hook]
pub fn use_rpc() -> Rc<jsonrpsee::core::client::Client> {
    let store = use_store_value::<RpcStorage>();
    store
        .rpc_client
        .clone()
        .expect_throw("RPC client is missing")
}

#[derive(Debug, Clone, Default, Store)]
pub struct RpcStorage {
    pub rpc_client: Option<Rc<jsonrpsee::core::client::Client>>,
}

impl PartialEq for RpcStorage {
    fn eq(&self, other: &Self) -> bool {
        self.rpc_client.is_some() == other.rpc_client.is_some()
    }
}
