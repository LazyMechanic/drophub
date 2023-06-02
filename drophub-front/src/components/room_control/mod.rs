mod client_list;
mod invite_list;
mod invite_modal;
mod room_info;
mod room_info_modal;

use std::ops::Deref;

use drophub::{InvitePassword, RoomInfo};
use yew::prelude::*;

use self::{
    client_list::ClientList, invite_list::InviteList, invite_modal::InviteModal,
    room_info::RoomInfo as RoomInfoComponent, room_info_modal::RoomInfoModal,
};
use crate::routes::room::state::ClientInfo;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
    pub room: RoomInfo,
    pub client: ClientInfo,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct State {
    menu: MenuState,
    selected_invite: Option<InvitePassword>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MenuState {
    Expanded,
    Minimized,
}

impl Default for MenuState {
    fn default() -> Self {
        MenuState::Expanded
    }
}

impl MenuState {
    fn next_state(&self) -> Self {
        match self {
            MenuState::Expanded => MenuState::Minimized,
            MenuState::Minimized => MenuState::Expanded,
        }
    }
}

#[function_component(RoomControl)]
pub fn room_control(props: &Props) -> Html {
    // TODO: animation on expanding/minimizing
    let state_handle = use_state(State::default);
    let min_exp_btn = {
        let minexp_onclick = Callback::from({
            let state_handle = state_handle.clone();
            move |_: MouseEvent| {
                let mut s = state_handle.deref().clone();
                s.menu = s.menu.next_state();
                state_handle.set(s);
            }
        });
        let icon = match state_handle.menu {
            MenuState::Expanded => html! { <i class="bi bi-caret-left"></i> },
            MenuState::Minimized => html! { <i class="bi bi-caret-right"></i> },
        };
        let classes = classes!(
            "btn",
            "btn-outline-light",
            "mt-auto",
            match state_handle.menu {
                MenuState::Expanded => Some("ms-auto"),
                MenuState::Minimized => None,
            },
        );

        html! {
            <button
                class={classes}
                type="button"
                onclick={minexp_onclick}
            >
                {icon}
            </button>
        }
    };

    let container_classes = classes!(
        "d-flex",
        "flex-column",
        "text-bg-secondary",
        "h-100",
        match state_handle.menu {
            MenuState::Expanded => &["p-3", "pt-2", "pb-2"][..],
            MenuState::Minimized => &["p-2"][..],
        },
        "gap-2",
    );

    html! {
        // TODO: change color by role
        <div class={container_classes}>
            <RoomInfoComponent
                placeholder={props.placeholder}
                menu_state={state_handle.menu}
                room_id={props.room.room_id}
            />
            <ClientList
                placeholder={props.placeholder}
                menu_state={state_handle.menu}
                clients={props.room.clients.clone()}
                host={props.room.host_id}
                cur_client={props.client.id}
            />
            <InviteList
                placeholder={props.placeholder}
                menu_state={state_handle.menu}
                invites={props.room.invites.clone()}
                room_cap={props.room.options.capacity}
                room_len={props.room.clients.len()}
            />
            {min_exp_btn}
            if let Some(selected_invite) = state_handle.selected_invite.clone() {
                <InviteModal
                    placeholder={props.placeholder}
                    room_id={props.room.room_id}
                    selected_invite={selected_invite}
                />
            }
            <RoomInfoModal
                placeholder={props.placeholder}
                room_id={props.room.room_id}
                room_opts={props.room.options.clone()}
                host={props.room.host_id}
            />
        </div>
    }
}
