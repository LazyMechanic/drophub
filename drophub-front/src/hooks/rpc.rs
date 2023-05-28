use yew::prelude::*;
use yewdux::prelude::*;

use crate::{rpc, rpc::RpcRequestTx};

#[hook]
pub fn use_rpc() -> RpcRequestTx {
    let s = use_store_value::<RpcStore>();
    s.tx.clone()
}

pub fn init_rpc(tx: RpcRequestTx) {
    Dispatch::<RpcStore>::new().set(RpcStore { tx })
}

#[derive(Debug, Clone, Store)]
struct RpcStore {
    tx: RpcRequestTx,
}

/// Always true
impl PartialEq for RpcStore {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Default for RpcStore {
    fn default() -> Self {
        Self {
            tx: rpc::channel().0,
        }
    }
}
