pub mod query;
pub mod state;

use std::{collections::HashMap, ops::Deref, rc::Rc, str::FromStr};

use drophub::{ClientEvent, InvitePassword, RoomId, RoomOptions, RoomRpcClient};
use jsonrpsee::core::client::Subscription;
use serde::{Deserialize, Deserializer, Serialize};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::{
    components::{RoomControl, RoomMediaShare},
    error::{Error, ShareError},
    hooks::{use_notify, use_rpc, NotifyProps},
    routes::{
        room::{
            query::{ActionConnect, ActionCreate, Query},
            state::{ClientRole, State},
        },
        Route,
    },
    unwrap_notify_ext::UnwrapNotifyExt,
};

#[function_component(Room)]
pub fn room() -> Html {
    let notify_manager = use_notify();
    let location = use_location().expect_notify(&notify_manager, "Failed to get location");
    let navigator = use_navigator().expect_notify(&notify_manager, "Failed to get navigator");
    let state_handle = use_state(State::default);
    //let rpc_client = use_rpc();

    //let room_handle = use_async(handle_room_update(rpc_client, state_handle.clone()));
    let room_handle = use_async(async { Ok::<(), ShareError>(()) });

    use_effect_with_deps(
        {
            let notify_manager = notify_manager.clone();
            let room_handle = room_handle.clone();
            move |_| {
                if let Some(err) = &room_handle.error {
                    notify_manager
                        .show_notify(NotifyProps::error(format!("Room handling failed: {err:?}")));

                    navigator.push(&Route::Home);
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
        <div class="container-fluid
                    d-flex
                    flex-row
                    h-100
                    p-3
                    gap-3
                    bg-body-secondary"
        >
            <RoomControl
                loading={state_handle.loading}
                room_id={state_handle.room.room_id}
                clients={
                    state_handle
                        .room
                        .clients
                        .iter()
                        .map(|id| (*id, if *id == state_handle.room.host_id { ClientRole::Host } else { ClientRole::Guest } ))
                        .collect::<HashMap<_, _>>()
                }
                cur_client={(state_handle.client.id, state_handle.client.role)}
                invites={state_handle.room.invites.clone()}
                capacity={state_handle.room.options.capacity}
            />
            <RoomMediaShare
                loading={state_handle.loading}
                medias={state_handle.room.files.clone()}
            />
        </div>
    }
}

async fn handle_room_update(
    rpc_client: Rc<jsonrpsee::core::client::Client>,
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
        }) => {
            let sub = rpc_client
                .create(RoomOptions {
                    encryption,
                    capacity,
                })
                .await
                .map_err(Error::from)?;

            let mut s = state_handle.deref().clone();
            s.client.role = ClientRole::Host;
            state_handle.set(s);

            handle_subscribe(sub, state_handle).await
        }
        Query::Connect(ActionConnect {
            room_id,
            invite_password,
        }) => {
            let sub = rpc_client
                .connect(room_id, invite_password)
                .await
                .map_err(Error::from)?;

            let mut s = state_handle.deref().clone();
            s.client.role = ClientRole::Guest;
            state_handle.set(s);

            handle_subscribe(sub, state_handle).await
        }
    }
}

async fn handle_subscribe(
    mut sub: Subscription<ClientEvent>,
    state_handle: UseStateHandle<State>,
) -> Result<(), ShareError> {
    while let Some(maybe_event) = sub.next().await {
        let event = maybe_event.map_err(Error::from)?;
        match event {
            ClientEvent::Init(jwt, client_id) => {
                let mut s = state_handle.deref().clone();
                s.client.jwt = jwt;
                s.client.id = client_id;
                state_handle.set(s);
            }
            ClientEvent::RoomInfo(room_info) => {
                let mut s = state_handle.deref().clone();
                s.room = room_info;
                state_handle.set(s);
            }
            ClientEvent::UploadRequest(_) => todo!("remove upload"),
        }
    }

    Ok(())
}
