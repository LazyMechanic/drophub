mod client_list;
mod invite_list;
mod invite_modal;
mod room_info;
mod room_info_modal;

use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yewdux::prelude::*;

use self::{
    client_list::ClientList, invite_list::InviteList, invite_modal::InviteModal,
    room_info::RoomInfo, room_info_modal::RoomInfoModal,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
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
    let state_handle = use_state(MenuState::default);
    let min_exp_btn = {
        let minexp_onclick = Callback::from({
            let state_handle = state_handle.clone();
            move |_: MouseEvent| state_handle.set(state_handle.next_state())
        });
        let icon = match *state_handle {
            MenuState::Expanded => html! { <i class="bi bi-caret-left"></i> },
            MenuState::Minimized => html! { <i class="bi bi-caret-right"></i> },
        };
        let classes = classes!(
            "btn",
            "btn-outline-light",
            "mt-auto",
            match *state_handle {
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
        match *state_handle {
            MenuState::Expanded => &["p-3", "pt-2", "pb-2"][..],
            MenuState::Minimized => &["p-2"][..],
        },
        "gap-2",
    );

    html! {
        // TODO: change color by role
        <div class={container_classes}>
            <RoomInfo placeholder={props.placeholder} menu_state={*state_handle} />
            <ClientList placeholder={props.placeholder} menu_state={*state_handle} />
            <InviteList placeholder={props.placeholder} menu_state={*state_handle} />
            {min_exp_btn}
            <InviteModal placeholder={props.placeholder} />
            <RoomInfoModal placeholder={props.placeholder} />
        </div>
    }
}
