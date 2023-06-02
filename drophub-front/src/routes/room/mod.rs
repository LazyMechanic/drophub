pub mod query;
pub mod state;

use std::{ops::Deref, str::FromStr};

use serde::{de::Error, Deserialize, Deserializer, Serialize};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::{
    components::{RoomControl, RoomFiles},
    hooks::{use_notify, use_rpc, NotifyProps},
    routes::room::{query::Query, state::State},
    unwrap_notify_ext::UnwrapNotifyExt,
};

#[function_component(Room)]
pub fn room() -> Html {
    let notify_manager = use_notify();
    let location = use_location().expect_notify(&notify_manager, "Failed to get location");
    let state_handle = use_state(State::default);
    let rpc_tx = use_rpc();

    // let init = use_async({
    //     let notify_manager = notify_manager.clone();
    //     let rpc_tx = rpc_tx.clone();
    //     async move { todo!() }
    // });

    use_effect_with_deps(
        {
            let location = location.clone();
            let state_handle = state_handle.clone();
            move |_| match location.query::<Query>() {
                Ok(query) => {
                    // TODO: send request on init
                    match query {
                        q @ Query::Create(_) => {
                            let mut s = State::placeholder_host().clone();
                            s.query = Some(q);
                            state_handle.set(s)
                        }
                        q @ Query::Connect(_) => {
                            let mut s = State::placeholder_guest().clone();
                            s.query = Some(q);
                            state_handle.set(s)
                        }
                    }
                }
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
