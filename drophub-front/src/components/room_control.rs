use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::Store;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub placeholder: bool,
}

#[function_component(RoomControl)]
pub fn room_control(props: &Props) -> Html {
    html! {
        <div
            class="d-flex
                   flex-column
                   text-bg-secondary
                   h-100
                   p-2
                   gap-2"
        >
            <RoomInfo placeholder={props.placeholder} />
            <ClientList placeholder={props.placeholder} />
            <button
                class="btn
                       btn-primary"
                type="button"
            >
                { "Invite" }
            </button>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct RoomInfoProps {
    #[prop_or_default]
    placeholder: bool,
}

#[function_component(RoomInfo)]
fn room_info(props: &RoomInfoProps) -> Html {
    let store = use_store_value::<Store>();

    let room_id = if props.placeholder {
        html! {
            <span class="placeholder
                         col-7"
            >{"123456"}</span>
        }
    } else {
        let room = store.room.as_ref().expect_throw("room not found");
        html! {
            <>
            {room.info.room_id}
            </>
        }
    };

    let host_id = if props.placeholder {
        html! {
            <span class="placeholder
                         col-12"
            >{"23e4567-e89b-12d3-a456-426614174000"}</span>
        }
    } else {
        let room = store.room.as_ref().expect_throw("room not found");
        html! {
            <>
            {room.info.host_id}
            </>
        }
    };

    html! {
        <ul class="list-group"
        >
            <li class="list-group-item">
                <div class="fw-bold">{ "Room ID:" }</div>
                {room_id}
            </li>
            <li class="list-group-item">
                <div class="fw-bold">{ "Host:" }</div>
                {host_id}
            </li>
        </ul>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct ClientListProps {
    #[prop_or_default]
    placeholder: bool,
}

#[function_component(ClientList)]
fn client_list(props: &ClientListProps) -> Html {
    let store = use_store_value::<Store>();

    let clients = if props.placeholder {
        let mut active = Some("active");
        std::iter::repeat_with(|| {
            let classes = classes!("list-group-item", "list-group-item-action", active.take());
            html! {
                <li class={classes}>
                    <span class="placeholder
                                 col-12
                                 text-nowrap"
                    >{"23e4567-e89b-12d3-a456-426614174000"}</span>
                </li>
            }
        })
        .take(5)
        .collect::<Html>()
    } else {
        let room = store.room.as_ref().expect_throw("room not found");
        room.info
            .clients
            .iter()
            .map(|client_id| {
                // TODO: highlight all owned files on hover
                let classes = classes!(
                    "list-group-item",
                    "list-group-item-action",
                    if client_id == &room.client_id {
                        Some("active")
                    } else {
                        None
                    }
                );
                html! {
                    <li class={classes}
                    >
                        {client_id}
                    </li>
                }
            })
            .collect::<Html>()
    };

    html! {
        <ul class="list-group">
            <li class="list-group-item">
                <div class="fw-bold">{ "Clients:" }</div>
            </li>
            {clients}
        </ul>
    }
}
