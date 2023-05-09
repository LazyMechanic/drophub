use drophub::{InvitePassword, RoomId};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub room_id: RoomId,
    pub invite_password: InvitePassword,
}

#[function_component(ConnectRoomLoad)]
pub fn connect_room_load(props: &Props) -> Html {
    html! {}
}
