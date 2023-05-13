use drophub::{ClientId, RoomId};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{room_control::MenuState, Placeholder},
    store::Store,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
    pub menu_state: MenuState,
}

#[function_component(RoomInfo)]
pub fn room_info(props: &Props) -> Html {
    let store = use_store_value::<Store>();
    let room = &store.room;

    let header = match props.menu_state {
        MenuState::Expanded => html! {
            <div class="fw-bold">
                <i class="bi bi-info-square me-2"></i>
                {"Room info:"}
            </div>
        },
        MenuState::Minimized => html! {
            <i class="bi bi-info-square text-center"></i>
        },
    };

    let room_id = match props.menu_state {
        MenuState::Expanded => html! {
            <li class="list-group-item">
                <div class="fw-bold">{ "Room ID:" }</div>
                <Placeholder<RoomId>
                    enabled={props.placeholder}
                    content={room.info.room_id}
                    size={7}
                />
            </li>
        },
        MenuState::Minimized => html! {
            // TODO: add tooltip
            <li class="list-group-item">
                <i class="bi bi-123"></i>
            </li>
        },
    };
    let host_id = match props.menu_state {
        MenuState::Expanded => html! {
            <li class="list-group-item">
                <div class="fw-bold">{ "Host ID:" }</div>
                <span class="font-monospace">
                    <Placeholder<ClientId>
                        enabled={props.placeholder}
                        content={room.info.host_id}
                        size={12}
                    />
                </span>
            </li>
        },
        MenuState::Minimized => html! {
            // TODO: add tooltip
            <li class="list-group-item">
                <i class="bi bi-person-check"></i>
            </li>
        },
    };

    html! {
        <div class="d-flex
                    flex-column
                    gap-2"
        >
            {header}
            <ul class="list-group shadow">
                {room_id}
                {host_id}
            </ul>
        </div>
    }
}
