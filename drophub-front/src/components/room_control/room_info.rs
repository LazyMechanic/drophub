use drophub::RoomId;
use yew::prelude::*;

use crate::components::{room_control::MenuState, Placeholder};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
    pub menu_state: MenuState,
    pub room_id: RoomId,
}

#[function_component(RoomInfo)]
pub fn room_info(props: &Props) -> Html {
    let header = match props.menu_state {
        MenuState::Expanded => html! {
            <div class="fw-bold">
                <i class="bi bi-info-square me-2"></i>
                {"Room "}
                <Placeholder<RoomId>
                    enabled={props.placeholder}
                    content={props.room_id}
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
