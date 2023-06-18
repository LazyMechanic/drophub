use drophub::RoomId;
use yew::{prelude::*, virtual_dom::ListenerKind::onclick};

use crate::components::Placeholder;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub room_id: RoomId,
    pub on_minmax: Callback<MouseEvent>,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    html! {
        <div
            class="d-flex
                   flex-row 
                   align-items-center"
        >
            <span class="fw-bold
                         dh-room-control-hidden"
            >
                {"Room "}
                <Placeholder<RoomId>
                    enabled={props.loading}
                    content={props.room_id}
                />
            </span>
            <button
                class="btn
                       btn-shade
                       ms-auto"
                type="button"
                onclick={&props.on_minmax}
            >
                <i
                    class="bi
                           bi-chevron-double-left
                           d-block"
                    id="dh-room-control-minmax-icon"
                ></i>
            </button>
        </div>
    }
}
