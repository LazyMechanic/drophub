use drophub::{ClientId, RoomId, RoomOptions};
use yew::prelude::*;

use crate::components::room_control2::room_info_modal::RoomInfoModal;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub room_id: RoomId,
    pub room_opts: RoomOptions,
    pub host: ClientId,
}

#[function_component(RoomInfo)]
pub fn room_info(props: &Props) -> Html {
    html! {
        <>
            <button
                class="btn
                       btn-shade
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
            <RoomInfoModal
                loading={props.loading}
                room_id={props.room_id}
                room_opts={props.room_opts.clone()}
                host={props.host}
            />
        </>
    }
}
