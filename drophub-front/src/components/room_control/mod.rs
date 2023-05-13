mod client_list;
mod invite_list;
mod invite_modal;
mod room_info;

use yew::prelude::*;
use yewdux::prelude::*;

use self::{
    client_list::ClientList, invite_list::InviteList, invite_modal::InviteModal,
    room_info::RoomInfo,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
}

#[function_component(RoomControl)]
pub fn room_control(props: &Props) -> Html {
    html! {
        // TODO: change color by role
        <div
            class="d-flex
                   flex-column
                   text-bg-secondary
                   h-100
                   p-3
                   gap-2"
        >
            <RoomInfo placeholder={props.placeholder} />
            <ClientList placeholder={props.placeholder} />
            <InviteList placeholder={props.placeholder} />
            <InviteModal placeholder={props.placeholder} />
        </div>
    }
}
