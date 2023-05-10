use drophub::{InvitePassword, RoomId};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    file_card::{FileCard, FileUpload},
    room_control::RoomControl,
    room_files::RoomFiles,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub room_id: RoomId,
    pub invite_password: InvitePassword,
}

#[function_component(ConnectRoomLoad)]
pub fn connect_room_load(props: &Props) -> Html {
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
