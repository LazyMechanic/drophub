use std::ops::Deref;

use drophub::{InviteId, RoomId};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{HtmlFormElement, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{routes::Route, validate::use_form_validation};

#[derive(Debug, Clone, Default)]
struct State {
    room_id: Option<RoomId>,
    invite_id: Option<InviteId>,
}

#[function_component(ConnectRoom)]
pub fn connect_room() -> Html {
    let form_node_ref = use_form_validation();

    let state_handle = use_state(State::default);

    let room_id_onchange = Callback::from({
        let state_handle = state_handle.clone();
        move |event: Event| {
            let value = event
                .target_dyn_into::<HtmlInputElement>()
                .expect_throw("failed to cast to HtmlInputElement")
                .value();

            let mut state = state_handle.deref().clone();
            let value_int = value
                .parse::<RoomId>()
                .expect_throw("failed to parse room id");
            state.room_id = Some(value_int);
            state_handle.set(state);
        }
    });
    let invite_id_onchange = Callback::from({
        let state_handle = state_handle.clone();
        move |event: Event| {
            let value = event
                .target_dyn_into::<HtmlInputElement>()
                .expect_throw("failed to cast to HtmlInputElement")
                .value();

            let mut state = state_handle.deref().clone();
            state.invite_id = Some(value);
            state_handle.set(state);
        }
    });

    let navigator = use_navigator().unwrap_throw();
    let form_onsubmit = Callback::from({
        let state_handle = state_handle.clone();
        move |event: SubmitEvent| {
            // TODO: send api request
            // TODO: validate form via check_validity
        }
    });
    let cancel_onclick = Callback::from(move |_| navigator.push(&Route::Home));

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
                            id="inviteIdInput"
                            type="password"
                            placeholder="qwerty123456"
                            required=true
                            onchange={invite_id_onchange}
                            value={state_handle.invite_id.clone()}
                        />
                        <label for="inviteIdInput">{ "Invite ID" }</label>
                        <div class="invalid-feedback">{ "Please provide valid invite ID." }</div>
                    </div>
                    <button
                        type="submit"
                        class="btn
                               btn-secondary"
                    >
                        { "Connect" }
                    </button>
                    <button
                        type="button"
                        class="btn
                               btn-outline-danger
                               ms-2"
                        onclick={cancel_onclick}
                    >
                        { "Cancel" }
                    </button>
                </form>
            </div>
        </div>
    }
}
