use std::ops::Deref;

use drophub::{InvitePassword, RoomId};
use wasm_bindgen::JsCast;
use web_sys::{HtmlFormElement, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    hooks::{use_form_validation, use_notify},
    routes::{
        room::query::{ActionConnect, ActionCreate, Query},
        Route,
    },
    unwrap_notify_ext::UnwrapNotifyExt,
};

#[derive(Debug, Clone, Default)]
struct ConnectState {
    room_id: Option<RoomId>,
    invite_password: Option<InvitePassword>,
}

const MIN_CAPACITY: usize = 2;
const MAX_CAPACITY: usize = 10;

#[derive(Debug, Clone, Eq, PartialEq)]
struct CreateState {
    capacity: usize,
    encryption: bool,
    is_loading: bool,
}

impl Default for CreateState {
    fn default() -> Self {
        Self {
            capacity: MIN_CAPACITY,
            encryption: false,
            is_loading: false,
        }
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let notify_manager = use_notify();
    let navigator = use_navigator().expect_notify(&notify_manager, "Failed to get navigator");

    let create_content = {
        let state_handle = use_state(CreateState::default);

        let form_node_ref = use_node_ref();
        let enc_node_ref = use_node_ref();

        let cap_oninput = Callback::from({
            let state_handle = state_handle.clone();
            let notify_manager = notify_manager.clone();
            move |input_event: InputEvent| {
                let event: Event = input_event
                    .dyn_into()
                    .expect_notify(&notify_manager, "Failed to cast capacity event to 'Event'");
                let input_elem: HtmlInputElement = event
                    .target()
                    .expect_notify(&notify_manager, "Capacity target not found")
                    .dyn_into()
                    .expect_notify(
                        &notify_manager,
                        "Failed to cast capacity Event to HtmlInputElement",
                    );
                let value = input_elem.value();
                let value_int: usize = value
                    .parse()
                    .expect_notify(&notify_manager, "Failed to parse capacity");

                let mut state = state_handle.deref().clone();
                state.capacity = value_int;
                state_handle.set(state);
            }
        });
        let enc_onclick = Callback::from({
            let state_handle = state_handle.clone();
            let enc_node_ref = enc_node_ref.clone();
            let notify_manager = notify_manager.clone();
            move |_| {
                let input_elem = enc_node_ref.cast::<HtmlInputElement>().expect_notify(
                    &notify_manager,
                    "Failed to cast encryption checkbox to 'HtmlInputElement'",
                );
                let mut state = state_handle.deref().clone();
                state.encryption = input_elem.checked();
                state_handle.set(state);
            }
        });

        let form_onsubmit = Callback::from({
            let state_handle = state_handle.clone();
            let navigator = navigator.clone();
            let notify_manager = notify_manager.clone();
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
                            &Query::Create(ActionCreate {
                                encryption: state_handle.encryption,
                                capacity: state_handle.capacity,
                            }),
                        )
                        .unwrap_notify(&notify_manager);
                }
            }
        });

        html! {
            <form
                onsubmit={form_onsubmit}
                ref={form_node_ref}
            >
                <div class="mb-3">
                    <label class="form-label" for="capacityRange">{ "Capacity" }</label>
                    <div class="d-flex
                                flex-row
                                align-items-center"
                    >
                        <input
                            class="form-control
                                   w-auto"
                            name="capacityInput"
                            type="number"
                            style="max-width: 70px;"
                            oninput={cap_oninput.clone()}
                            min={MIN_CAPACITY.to_string()}
                            max={MAX_CAPACITY.to_string()}
                            value={state_handle.capacity.to_string()}
                        />
                        <input
                            class="form-range
                                   ms-3"
                            id="capacityRange"
                            name="capacityRange"
                            type="range"
                            oninput={cap_oninput}
                            min={MIN_CAPACITY.to_string()}
                            max={MAX_CAPACITY.to_string()}
                            value={state_handle.capacity.to_string()}
                        />
                    </div>
                </div>
                <div class="mb-3 form-check form-switch">
                    <input
                        class="form-check-input"
                        id="encryptionCheck"
                        type="checkbox"
                        role="switch"
                        disabled=true
                        checked={state_handle.encryption}
                        onclick={enc_onclick}
                        ref={enc_node_ref}
                    />
                    <label class="form-check-label" for="encryptionCheck">{ "Encryption" }</label>
                </div>
                <button
                    type="submit"
                    class="btn
                           btn-primary"
                >
                    { "Create" }
                </button>
            </form>
        }
    };

    let connect_content = {
        let state_handle = use_state(ConnectState::default);
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
                                room_id: state_handle.room_id.unwrap_notify(&notify_manager),
                                invite_password: state_handle
                                    .invite_password
                                    .clone()
                                    .unwrap_notify(&notify_manager),
                            }),
                        )
                        .unwrap_notify(&notify_manager);
                }
            }
        });

        html! {
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
                    <label for="invitePasswordInput">{ "Invite password" }</label>
                    <div class="invalid-feedback">{ "Please provide valid invite password." }</div>
                </div>
                <button
                    type="submit"
                    class="btn
                           btn-secondary"
                >
                    { "Connect" }
                </button>
            </form>
        }
    };

    html! {
        <div class="container-fluid
                    h-100
                    pt-4
                    pb-4
                    bg-secondary"
        >
            <div
                class="bg-body
                       border
                       border-0
                       rounded
                       shadow
                       mx-auto"
                style="max-width: 540px;"
            >
                <ul
                    class="nav
                           nav-tabs
                           nav-justified"
                    id="dh-home-tab"
                    role="tablist"
                >
                    <li class="nav-item" role="presentation">
                        <button
                            class="nav-link
                                   active
                                   dh-home-tab-btn
                                   text-dark"
                            id="dh-home-create-tab"
                            data-bs-toggle="tab"
                            data-bs-target="#dh-home-create-tab-pane"
                            type="button"
                            role="tab"
                            aria-controls="dh-home-create-tab-pane"
                            aria-selected="true"
                        >
                            {"Create room"}
                        </button>
                    </li>
                    <li class="nav-item" role="presentation">
                        <button
                            class="nav-link
                                   dh-home-tab-btn
                                   text-dark"
                            id="dh-home-connect-tab"
                            data-bs-toggle="tab"
                            data-bs-target="#dh-home-connect-tab-pane"
                            type="button"
                            role="tab"
                            aria-controls="dh-home-connect-tab-pane"
                            aria-selected="false"
                        >
                            {"Connect to room"}
                        </button>
                    </li>
                </ul>
                <div
                    class="tab-content
                           border-start
                           border-end
                           border-bottom
                           border-1
                           rounded-bottom
                           bg-body" 
                    id="dh-home-tab-content"
                >
                    <div
                        class="tab-pane
                               fade
                               show
                               active
                               p-3"
                        id="dh-home-create-tab-pane"
                        role="tabpanel"
                        aria-labelledby="profile-tab"
                        tabindex="0"
                    >
                        {create_content}
                    </div>
                    <div
                        class="tab-pane
                               fade
                               p-3"
                        id="dh-home-connect-tab-pane"
                        role="tabpanel"
                        aria-labelledby="home-tab"
                        tabindex="0"
                    >
                        {connect_content}
                    </div>
                </div>
            </div>
        </div>
    }
}
