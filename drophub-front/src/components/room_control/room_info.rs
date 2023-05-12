use drophub::{ClientId, RoomId};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{components::Placeholder, store::Store};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
}

#[function_component(RoomInfo)]
pub fn room_info(props: &Props) -> Html {
    let store = use_store_value::<Store>();
    let room = &store.room;

    html! {
        <div class="d-flex flex-column gap-2">
            <div class="fw-bold">{"Room info:"}</div>
            <ul class="list-group shadow"
            >
                <li class="list-group-item">
                    <div class="fw-bold">{ "Room ID:" }</div>
                    <Placeholder<RoomId>
                        enabled={props.placeholder}
                        content={room.info.room_id}
                        size={7}
                    />
                </li>
                <li class="list-group-item">
                    <div class="fw-bold">{ "Host:" }</div>
                    <span class="font-monospace">
                        <Placeholder<ClientId>
                            enabled={props.placeholder}
                            content={room.info.host_id}
                            size={12}
                        />
                    </span>
                </li>
            </ul>
        </div>
    }
}
