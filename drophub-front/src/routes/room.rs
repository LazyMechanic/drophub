use std::str::FromStr;

use serde::{de::Error, Deserialize, Deserializer, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{RoomControl, RoomFiles},
    hooks::{use_notify, use_room_store, ClientRole, NotifyProps, RoomState},
    unwrap_notify_ext::UnwrapNotifyExt,
};

fn from_str<'de, D, S>(deserializer: D) -> Result<S, D::Error>
where
    D: Deserializer<'de>,
    S: FromStr,
{
    let s = <&str as Deserialize>::deserialize(deserializer)?;
    S::from_str(&s).map_err(|_| D::Error::custom("could not parse string"))
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum Query {
    Create(ActionCreate),
    Connect(ActionConnect),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActionCreate {
    #[serde(deserialize_with = "from_str")]
    pub encryption: bool,
    #[serde(deserialize_with = "from_str")]
    pub capacity: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActionConnect {
    #[serde(deserialize_with = "from_str")]
    pub room_id: u64,
    pub invite_password: String,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct State {
    query: Option<Query>,
}

#[function_component(Room)]
pub fn room() -> Html {
    let state_handle = use_state(State::default);
    let (room_store, room_store_dispatch) = use_room_store();
    let notify_manager = use_notify();
    let location = use_location().expect_notify(&notify_manager, "Failed to get location");

    use_effect_with_deps(
        {
            let location = location.clone();
            let state_handle = state_handle.clone();
            let room_store_dispatch = room_store_dispatch.clone();
            move |_| match location.query::<Query>() {
                Ok(q) => {
                    // TODO: send request on init
                    match &q {
                        Query::Create(_) => room_store_dispatch.reduce_mut(|s| {
                            s.room = RoomState::placeholder(ClientRole::Host).clone()
                        }),
                        Query::Connect(_) => room_store_dispatch.reduce_mut(|s| {
                            s.room = RoomState::placeholder(ClientRole::Guest).clone()
                        }),
                    }

                    state_handle.set(State { query: Some(q) })
                }
                Err(q_err) => {
                    state_handle.set(State { query: None });
                    notify_manager.show_notify(NotifyProps::error(format!(
                        "Failed to parse URL query: {q_err:?}"
                    )));
                }
            }
        },
        location.query_str().to_owned(),
    );

    let placeholder = state_handle.query.is_none() || true /* loading */;

    html! {
        <div class="d-flex
                    flex-row
                    h-100
                    placeholder-glow"
        >
            <RoomControl {placeholder} />
            <RoomFiles {placeholder} />
        </div>
    }
}
