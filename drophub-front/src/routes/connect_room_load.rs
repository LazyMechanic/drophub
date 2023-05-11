use drophub::{InvitePassword, RoomId};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{room_control::RoomControl, room_files::RoomFiles},
    store::{Room, Store},
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub room_id: RoomId,
    pub invite_password: InvitePassword,
}

#[function_component(ConnectRoomLoad)]
pub fn connect_room_load(props: &Props) -> Html {
    let (_store, store_dispatch) = use_store::<Store>();
    store_dispatch.reduce_mut(|store| store.room = Room::placeholder_guest().clone());

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
