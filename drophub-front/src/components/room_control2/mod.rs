mod client_list;
mod header;
mod invite_list;
mod invite_modal;
mod room_info;
mod room_info_modal;

use std::collections::HashMap;

use drophub::{ClientId, InvitePassword, RoomId};
use web_sys::Element;
use yew::prelude::*;

use self::{client_list::ClientList, header::Header, invite_list::InviteList, room_info::RoomInfo};
use crate::{
    components::Placeholder, hooks::use_notify, routes::room::state::ClientRole,
    unwrap_notify_ext::UnwrapNotifyExt,
};

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
    let notify_manager = use_notify();
    let container_node_ref = use_node_ref();

    let on_minmax = Callback::from({
        let container_node_ref = container_node_ref.clone();
        move |_| {
            let elem = container_node_ref
                .cast::<Element>()
                .expect_notify(&notify_manager, "Failed to cast 'NodeRef' to 'Element'");

            elem.class_list()
                .toggle("dh-room-control-minimized")
                .expect_notify(
                    &notify_manager,
                    "Failed to toggle 'dh-room-control-minimized' class",
                );
        }
    });

    let mut props = props.clone();
    props.loading = false;

    html! {
        <div class="overflow-scroll-marker
                    overflow-scroll-marker-body
                    flex-grow-0
                    flex-shrink-0
                    border
                    border-0
                    rounded">
            <div
                class="d-flex
                       flex-column
                       align-items-stretch
                       border
                       border-0
                       rounded
                       bg-body
                       shadow
                       p-3
                       gap-2
                       h-100
                       text-nowrap
                       overflow-y-auto
                       dh-room-control"
                ref={container_node_ref}
            >
                <Header
                    loading={props.loading}
                    room_id={props.room_id}
                    on_minmax={on_minmax}
                />
                <hr class="my-1" />
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
        </div>
    }
}
