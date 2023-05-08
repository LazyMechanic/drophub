use std::rc::Rc;

use drophub::{ClientId, JwtEncoded, RoomInfo};
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

use crate::rpc::RpcRequestTx;

pub type ContextHandle = UseReducerHandle<Context>;

#[derive(Debug, Clone)]
pub struct Context {
    // For PartialEq purpose
    id: Uuid,
    pub rpc_tx: RpcRequestTx,
    pub room: Option<Room>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Room {
    pub jwt: JwtEncoded,
    pub info: RoomInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextAction {
    ConnectRoom { jwt: JwtEncoded, info: RoomInfo },
    DisconnectRoom,
}

impl Context {
    pub fn new(rpc_tx: RpcRequestTx) -> Self {
        Self {
            id: Uuid::new_v4(),
            rpc_tx,
            room: None,
        }
    }
}

impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Reducible for Context {
    type Action = ContextAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ContextAction::ConnectRoom { jwt, info } => Self {
                id: self.id,
                rpc_tx: self.rpc_tx.clone(),
                room: Some(Room { jwt, info }),
            }
            .into(),
            ContextAction::DisconnectRoom => Self {
                id: self.id,
                rpc_tx: self.rpc_tx.clone(),
                room: None,
            }
            .into(),
        }
    }
}

#[hook]
pub fn use_app_context() -> ContextHandle {
    use_context::<ContextHandle>().expect_throw("Context not found")
}
