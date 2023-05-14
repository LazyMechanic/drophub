use serde::{Deserialize, Serialize};
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{room_control::RoomControl, room_files::RoomFiles},
    store::{Room, Store},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub encryption: bool,
    pub capacity: usize,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {}

#[function_component(CreateRoomLoad)]
pub fn create_room_load(props: &Props) -> Html {
    let (_store, store_dispatch) = use_store::<Store>();
    store_dispatch.reduce_mut(|store| store.room = Room::placeholder_host().clone());
    let location = use_location().unwrap();
    let query = location
        .query::<Query>()
        .expect_throw("failed to parse query params");

    // TODO: send api request
    html! {
        <div class="d-flex
                    flex-row
                    h-100
                    placeholder-glow"
        >
            <RoomControl placeholder={true} />
            <RoomFiles placeholder={true} />
        </div>
    }
}
