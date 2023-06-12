use drophub::RoomId;
use yew::prelude::*;

use crate::components::Placeholder;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub room_id: RoomId,
}

#[function_component(RoomInfo)]
pub fn room_info(props: &Props) -> Html {
    html! {
        <div class="d-flex
                    flex-column
                    gap-2"
        >
            <div class="fw-bold fs-6">
                <i class="bi
                          bi-info-square
                          dh-room-control-icon"
                ></i>
                <span class="dh-room-control-text ms-1">
                    {"Room "}
                    <Placeholder<RoomId>
                        enabled={props.loading}
                        content={props.room_id}
                    />
                </span>
            </div>
            <button
                class="btn
                       btn-secondary
                       shadow"
                type="button"
                data-bs-toggle="modal"
                data-bs-target="#dh-room-control-room-info-modal"
            >
                <i class="bi
                          bi-sliders
                          dh-room-control-icon"
                ></i>
                <span class="dh-room-control-text ms-1">
                    {"Info"}
                </span>
            </button>
        </div>
    }
}
