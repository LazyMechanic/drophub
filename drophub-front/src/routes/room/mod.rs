pub mod query;
pub mod state;

use std::{ops::Deref, str::FromStr};

use drophub::{InvitePassword, RoomId, RoomOptions};
use serde::{Deserialize, Deserializer, Serialize};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::{
    components::{RoomControl, RoomFiles},
    error::{Error, ShareError},
    hooks::{use_notify, use_rpc, NotifyManager, NotifyProps},
    routes::room::{
        query::{ActionConnect, ActionCreate, Query},
        state::{ClientRole, State},
    },
    rpc::{RoomCredentials, RoomMsg, RpcRequestTx, RpcSubscribeRequest, RpcSubscribeResponse},
    unwrap_notify_ext::UnwrapNotifyExt,
};

#[function_component(Room)]
pub fn room() -> Html {
    let notify_manager = use_notify();
    let location = use_location().expect_notify(&notify_manager, "Failed to get location");
    let state_handle = use_state(State::default);
    let rpc_tx = use_rpc();

    let room_handle = use_async(handle_room_update(rpc_tx.clone(), state_handle.clone()));

    use_effect_with_deps(
        {
            let notify_manager = notify_manager.clone();
            let room_handle = room_handle.clone();
            move |_| {
                if let Some(err) = &room_handle.error {
                    notify_manager
                        .show_notify(NotifyProps::error(format!("Room handling failed: {err:?}")));
                }
            }
        },
        room_handle.error.is_some(),
    );

    use_effect_with_deps(
        {
            let location = location.clone();
            let state_handle = state_handle.clone();
            move |_| match location.query::<Query>() {
                Ok(query) => match query {
                    q @ Query::Create(_) => {
                        let mut s = State::placeholder_host().clone();
                        s.query = Some(q);
                        s.loading = true;
                        state_handle.set(s);

                        room_handle.run();
                    }
                    q @ Query::Connect(_) => {
                        let mut s = State::placeholder_guest().clone();
                        s.query = Some(q);
                        s.loading = true;
                        state_handle.set(s);

                        room_handle.run();
                    }
                },
                Err(q_err) => {
                    let mut s = state_handle.deref().clone();
                    s.query = None;
                    s.loading = true;
                    state_handle.set(s);

                    notify_manager.show_notify(NotifyProps::error(format!(
                        "Failed to parse URL query: {q_err:?}"
                    )));
                }
            }
        },
        location.query_str().to_owned(),
    );

    html! {
        <div class="d-flex
                    flex-row
                    h-100
                    placeholder-glow"
        >
            <RoomControl
                placeholder={state_handle.loading}
                room={state_handle.room.clone()}
                client={state_handle.client.clone()}
            />
            <RoomFiles
                placeholder={state_handle.loading}
                files={state_handle.room.files.clone()}
            />
        </div>
    }
}

async fn handle_room_update(
    rpc_tx: RpcRequestTx,
    state_handle: UseStateHandle<State>,
) -> Result<(), ShareError> {
    let query = state_handle
        .deref()
        .query
        .clone()
        .ok_or_else(|| Error::Other(anyhow::anyhow!("Failed to get query")))?;

    match query {
        Query::Create(ActionCreate {
            encryption,
            capacity,
        }) => handle_host(rpc_tx, state_handle, encryption, capacity).await,
        Query::Connect(ActionConnect {
            room_id,
            invite_password,
        }) => handle_guest(rpc_tx, state_handle, room_id, invite_password).await,
    }
}

async fn handle_host(
    rpc_tx: RpcRequestTx,
    state_handle: UseStateHandle<State>,
    encryption: bool,
    capacity: usize,
) -> Result<(), ShareError> {
    let mut sub_rx = rpc_tx.sub(RpcSubscribeRequest::CreateRoom(RoomOptions {
        encryption,
        capacity,
    }))?;

    while let Some(resp) = sub_rx.recv().await {
        match resp {
            RpcSubscribeResponse::Room(room_msg) => match room_msg {
                RoomMsg::Init(jwt, client_id) => {
                    let mut s = state_handle.deref().clone();
                    s.client.id = client_id;
                    s.client.jwt = jwt;
                    s.client.role = ClientRole::Host;
                    state_handle.set(s);
                }
                RoomMsg::RoomInfo(room_info) => {
                    let mut s = state_handle.deref().clone();
                    s.room = room_info;
                    s.loading = false;
                    state_handle.set(s);
                }
                RoomMsg::UploadRequest(_) => unimplemented!(),
            },
        }
    }

    Ok(())
}

async fn handle_guest(
    rpc_tx: RpcRequestTx,
    state_handle: UseStateHandle<State>,
    room_id: RoomId,
    invite_password: InvitePassword,
) -> Result<(), ShareError> {
    let mut sub_rx = rpc_tx.sub(RpcSubscribeRequest::ConnectRoom(RoomCredentials {
        id: room_id,
        password: invite_password,
    }))?;

    while let Some(resp) = sub_rx.recv().await {
        match resp {
            RpcSubscribeResponse::Room(room_msg) => match room_msg {
                RoomMsg::Init(jwt, client_id) => {
                    let mut s = state_handle.deref().clone();
                    s.client.id = client_id;
                    s.client.jwt = jwt;
                    s.client.role = ClientRole::Host;
                    state_handle.set(s);
                }
                RoomMsg::RoomInfo(room_info) => {
                    let mut s = state_handle.deref().clone();
                    s.room = room_info;
                    s.loading = false;
                    state_handle.set(s);
                }
                RoomMsg::UploadRequest(_) => unimplemented!(),
            },
        }
    }

    Ok(())
}
