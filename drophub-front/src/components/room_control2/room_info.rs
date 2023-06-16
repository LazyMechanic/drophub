use drophub::RoomId;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub room_id: RoomId,
}

#[function_component(RoomInfo)]
pub fn room_info(props: &Props) -> Html {
    html! {
        <button
            class="btn
                   btn-body
                   d-flex
                   flex-row "
            type="button"
            data-bs-toggle="modal"
            data-bs-target="#dh-room-control-room-info-modal"
        >
            <i class="bi
                      bi-info-square"
            ></i>
            <span class="d-inline-block
                         ms-2
                         me-auto
                         dh-room-control-hidden"
            >
                {"Room info"}
            </span>
        </button>
    }
}
