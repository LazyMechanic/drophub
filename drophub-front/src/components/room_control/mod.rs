mod client_list;
mod invite_list;
mod invite_modal;
mod room_info;
mod room_info_modal;

use std::ops::Deref;

use drophub::{InvitePassword, RoomInfo};
use web_sys::HtmlButtonElement;
use yew::prelude::*;
use yew_hooks::use_toggle;

use self::{
    client_list::ClientList, invite_list::InviteList, invite_modal::InviteModal,
    room_info::RoomInfo as RoomInfoComponent, room_info_modal::RoomInfoModal,
};
use crate::{
    hooks::use_notify, routes::room::state::ClientInfo, unwrap_notify_ext::UnwrapNotifyExt,
};

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
    fn toggle(&self) -> Self {
        match self {
            MenuState::Expanded => MenuState::Minimized,
            MenuState::Minimized => MenuState::Expanded,
        }
    }
}

#[function_component(RoomControl)]
pub fn room_control(props: &Props) -> Html {
    let notify_manager = use_notify();

    // TODO: animation on expanding/minimizing
    let menu_state_handle = use_toggle(MenuState::Expanded, MenuState::Minimized);
    let selected_invite_handle = use_state(|| None);

    let min_exp_btn = {
        let minexp_onclick = Callback::from({
            let menu_state_handle = menu_state_handle.clone();
            move |_: MouseEvent| menu_state_handle.toggle()
        });
        let icon = match *menu_state_handle {
            MenuState::Expanded => html! { <i class="bi bi-caret-left"></i> },
            MenuState::Minimized => html! { <i class="bi bi-caret-right"></i> },
        };
        let classes = classes!(
            "btn",
            "btn-outline-light",
            "mt-auto",
            match *menu_state_handle {
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

    let invite_onclick = Callback::from({
        let notify_manager = notify_manager.clone();
        let selected_invite_handle = selected_invite_handle.clone();
        move |e: MouseEvent| {
            let btn = e.target_dyn_into::<HtmlButtonElement>().expect_notify(
                &notify_manager,
                "Failed to convert 'MouseEvent' target to 'HtmlButtonElement'",
            );

            let invite_password = btn.text_content();
            selected_invite_handle.set(invite_password)
        }
    });

    let container_classes = classes!(
        "d-flex",
        "flex-column",
        "text-bg-secondary",
        "h-100",
        match *menu_state_handle {
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
                menu_state={*menu_state_handle}
                room_id={props.room.room_id}
            />
            <ClientList
                placeholder={props.placeholder}
                menu_state={*menu_state_handle}
                clients={props.room.clients.clone()}
                host={props.room.host_id}
                cur_client={props.client.id}
            />
            <InviteList
                placeholder={props.placeholder}
                menu_state={*menu_state_handle}
                invites={props.room.invites.clone()}
                room_cap={props.room.options.capacity}
                room_len={props.room.clients.len()}
                invite_onclick={invite_onclick}
            />
            {min_exp_btn}
            <InviteModal
                placeholder={props.placeholder}
                room_id={props.room.room_id}
                selected_invite={selected_invite_handle.deref().clone().unwrap_or_else(|| "password".to_owned())}
            />
            <RoomInfoModal
                placeholder={props.placeholder}
                room_id={props.room.room_id}
                room_opts={props.room.options.clone()}
                host={props.room.host_id}
            />
        </div>
    }
}
