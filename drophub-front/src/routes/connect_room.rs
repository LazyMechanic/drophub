use drophub::{InvitePassword, RoomId};
use yew::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub room_id: RoomId,
    pub invite_password: InvitePassword,
}

#[function_component(ConnectRoom)]
pub fn connect_room(props: &Props) -> Html {
    todo!()
}
