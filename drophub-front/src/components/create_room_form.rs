use std::ops::Deref;

use wasm_bindgen::JsCast;
use web_sys::{HtmlFormElement, HtmlInputElement};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{
    hooks::use_notify,
    routes::{create_room::Query, Route},
    unwrap_notify_ext::UnwrapNotifyExt,
};

const MIN_CAPACITY: usize = 2;
const MAX_CAPACITY: usize = 10;

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    capacity: usize,
    encryption: bool,
    is_loading: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            capacity: MIN_CAPACITY,
            encryption: false,
            is_loading: false,
        }
    }
}

#[function_component(CreateRoomForm)]
pub fn create_room_form() -> Html {
    let notify_manager = use_notify();
    let navigator = use_navigator().expect_notify(&notify_manager, "Failed to get navigator");

    let state_handle = use_state(State::default);

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
                        &Route::CreateRoom,
                        &Query {
                            encryption: state_handle.encryption,
                            capacity: state_handle.capacity,
                        },
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
            onsubmit={form_onsubmit}
            ref={form_node_ref}
        >
            <div>
                <label class="form-label" for="capacityRange">{ "Capacity" }</label>
                <div class="d-flex
                            flex-row
                            align-items-center
                            gap-3"
                >
                    <input
                        class="form-control"
                        name="capacityInput"
                        type="number"
                        style="max-width: 70px;"
                        oninput={cap_oninput.clone()}
                        min={MIN_CAPACITY.to_string()}
                        max={MAX_CAPACITY.to_string()}
                        value={state_handle.capacity.to_string()}
                    />
                    <input
                        class="form-range"
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
            <div class="form-check form-switch">
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
}
