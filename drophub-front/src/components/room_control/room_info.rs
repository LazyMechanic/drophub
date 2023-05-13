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
                {"Room "}
                <Placeholder<RoomId>
                    enabled={props.placeholder}
                    content={room.info.room_id}
                />
            </div>
        },
        MenuState::Minimized => html! {
            <i class="bi bi-info-square text-center"></i>
        },
    };

    let info_btn = {
        let content = match props.menu_state {
            MenuState::Expanded => html! { <>{"Info"}</>},
            MenuState::Minimized => html! { <i class="bi bi-sliders"></i> },
        };

        html! {
            <button
                class="btn
                       btn-light"
                type="button"
                data-bs-toggle="modal"
                data-bs-target="#roomInfoModal"
            >
                {content}
            </button>
        }
    };

    html! {
        <div class="d-flex
                    flex-column
                    gap-2"
        >
            {header}
            {info_btn}
        </div>
    }
}
