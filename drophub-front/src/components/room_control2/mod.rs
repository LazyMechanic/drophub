mod client_list;
mod invite_list;
mod invite_modal;
mod room_info;
mod room_info_modal;

use std::collections::HashMap;

use drophub::{ClientId, InvitePassword, RoomId};
use yew::prelude::*;

use self::{client_list::ClientList, invite_list::InviteList, room_info::RoomInfo};
use crate::routes::room::state::ClientRole;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub room_id: RoomId,
    pub clients: HashMap<ClientId, ClientRole>,
    pub cur_client: (ClientId, ClientRole),
    pub invites: Vec<InvitePassword>,
    pub capacity: usize,
}

#[function_component(RoomControl)]
pub fn room_control(props: &Props) -> Html {
    html! {
        <div
            class="d-flex
                   flex-column
                   bg-body
                   border
                   border-0
                   rounded
                   shadow
                   h-100
                   p-3
                   gap-2
                   dh-room-control"
        >
            <RoomInfo
                loading={props.loading}
                room_id={props.room_id}
            />
            <ClientList
                loading={props.loading}
                clients={props.clients.clone()}
                cur_client={props.cur_client.clone()}
                capacity={props.capacity}
            />
            <InviteList
                loading={props.loading}
                invites={props.invites.clone()}
                capacity={props.capacity}
                clients_count={props.clients.len()}
            />
        </div>
    }
}
