use std::ops::Deref;

use drophub::{InvitePassword, RoomId};
use web_sys::{HtmlFormElement, HtmlInputElement};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    hooks::{use_form_validation, use_notify},
    routes::{
        room::query::{ActionConnect, Query},
        Route,
    },
    unwrap_notify_ext::UnwrapNotifyExt,
};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct State {
    room_id: Option<RoomId>,
    invite_password: Option<InvitePassword>,
}

#[function_component(ConnectRoomForm)]
pub fn connect_room_form() -> Html {
    let notify_manager = use_notify();
    let navigator = use_navigator().expect_notify(&notify_manager, "Failed to get navigator");

    let state_handle = use_state(State::default);
    let form_node_ref = use_form_validation();

    let room_id_onchange = Callback::from({
        let state_handle = state_handle.clone();
        let notify_manager = notify_manager.clone();
        move |event: Event| {
            let value = event
                .target_dyn_into::<HtmlInputElement>()
                .expect_notify(
                    &notify_manager,
                    "Failed to cast 'Event' to 'HtmlInputElement'",
                )
                .value();

            let mut state = state_handle.deref().clone();
            let value_int = value
                .parse::<RoomId>()
                .expect_notify(&notify_manager, "Failed to parse room id");
            state.room_id = Some(value_int);
            state_handle.set(state);
        }
    });

    let invite_password_onchange = Callback::from({
        let state_handle = state_handle.clone();
        let notify_manager = notify_manager.clone();
        move |event: Event| {
            let value = event
                .target_dyn_into::<HtmlInputElement>()
                .expect_notify(&notify_manager, "Failed to cast to 'HtmlInputElement'")
                .value();

            let mut state = state_handle.deref().clone();
            state.invite_password = Some(value);
            state_handle.set(state);
        }
    });

    let form_onsubmit = Callback::from({
        let state_handle = state_handle.clone();
        let notify_manager = notify_manager.clone();
        let navigator = navigator.clone();
        let form_node_ref = form_node_ref.clone();
        move |event: SubmitEvent| {
            event.prevent_default();
            event.stop_propagation();

            let elem = form_node_ref
                .cast::<HtmlFormElement>()
                .expect_notify(&notify_manager, "Failed to cast to 'HtmlFormElement'");

            if elem.check_validity() {
                navigator
                    .push_with_query(
                        &Route::Room,
                        &Query::Connect(ActionConnect {
                            room_id: state_handle
                                .room_id
                                .expect_notify(&notify_manager, "Room id is missing"),
                            invite_password: state_handle
                                .invite_password
                                .clone()
                                .expect_notify(&notify_manager, "Invite password is missing"),
                        }),
                    )
                    .unwrap_notify(&notify_manager);
            }
        }
    });

    html! {
        <form
            class="d-flex
                   flex-column
                   gap-3"
            novalidate=true
            ref={form_node_ref}
            onsubmit={form_onsubmit}
        >
            <div class="form-floating">
                <input
                    class="form-control"
                    id="roomIdInput"
                    type="number"
                    placeholder="123456"
                    required=true
                    onchange={room_id_onchange}
                    value={state_handle.room_id.map(|v| v.to_string())}
                />
                <label for="roomIdInput">{ "Room ID" }</label>
                <div class="invalid-feedback">{ "Please provide valid room ID." }</div>
            </div>
            <div class="form-floating">
                <input
                    class="form-control"
                    id="invitePasswordInput"
                    type="password"
                    placeholder="qwerty123456"
                    required=true
                    onchange={invite_password_onchange}
                    value={state_handle.invite_password.clone()}
                />
                <label for="invitePasswordInput">{ "Invite password" }</label>
                <div class="invalid-feedback">{ "Please provide valid invite password." }</div>
            </div>
            <button
                type="submit"
                class="btn
                       btn-primary"
            >
                { "Connect" }
            </button>
        </form>
    }
}
