use std::ops::Deref;

use drophub::{InvitePassword, RoomId};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{HtmlFormElement, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    hooks::use_notify,
    routes::{
        room::{ActionConnect, Query},
        Route,
    },
    unwrap_notify_ext::UnwrapNotifyExt,
    validate::use_form_validation,
};

#[derive(Debug, Clone, Default)]
struct State {
    room_id: Option<RoomId>,
    invite_password: Option<InvitePassword>,
}

#[function_component(ConnectRoom)]
pub fn connect_room() -> Html {
    let state_handle = use_state(State::default);
    let notify_manager = use_notify();

    let form_node_ref = use_form_validation();

    let room_id_onchange = Callback::from({
        let state_handle = state_handle.clone();
        let notify_manager = notify_manager.clone();
        move |event: Event| {
            let value = event
                .target_dyn_into::<HtmlInputElement>()
                .expect_notify(&notify_manager, "Failed to cast to 'HtmlInputElement'")
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

    let navigator = use_navigator().unwrap_throw();
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
                            room_id: state_handle.room_id.unwrap_throw(),
                            invite_password: state_handle.invite_password.clone().unwrap_throw(),
                        }),
                    )
                    .unwrap_notify(&notify_manager);
            }
        }
    });

    html! {
        <div class="container
                    position-relative
                    top-50
                    translate-middle-y"
        >
            <h2 class="pb-2">{ "Enter invite credentials" }</h2>
            <div class="border
                        border-2
                        rounded
                        p-3"
            >
                <form
                    novalidate=true
                    ref={form_node_ref}
                    onsubmit={form_onsubmit}
                >
                    <div class="mb-3 form-floating">
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
                    <div class="mb-3 form-floating">
                        <input
                            class="form-control"
                            id="invitePasswordInput"
                            type="password"
                            placeholder="qwerty123456"
                            required=true
                            onchange={invite_password_onchange}
                            value={state_handle.invite_password.clone()}
                        />
                        <label for="invitePasswordInput">{ "Invite ID" }</label>
                        <div class="invalid-feedback">{ "Please provide valid invite ID." }</div>
                    </div>
                    <button
                        type="submit"
                        class="btn
                               btn-secondary"
                    >
                        { "Connect" }
                    </button>
                </form>
            </div>
        </div>
    }
}
