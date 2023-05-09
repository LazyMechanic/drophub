use std::ops::Deref;

use time::Duration;
use tracing::instrument;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::alert::AlertKind,
    ctx::use_app_context,
    routes::Route,
    store,
    store::{AlertProps, Store},
};

const MIN_CAPACITY: usize = 2;
const MAX_CAPACITY: usize = 10;

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    capacity: usize,
    encryption: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            capacity: MIN_CAPACITY,
            encryption: false,
        }
    }
}

#[function_component(CreateRoom)]
#[instrument]
pub fn create_room() -> Html {
    let ctx_handle = use_app_context();
    let state_handle = use_state(State::default);
    let (store, store_dispatch) = use_store::<Store>();

    let cap_oninput = Callback::from({
        let state_handle = state_handle.clone();
        move |input_event: InputEvent| {
            let event: Event = input_event.dyn_into().unwrap_throw();
            let input_elem: HtmlInputElement =
                event.target().unwrap_throw().dyn_into().unwrap_throw();
            let value = input_elem.value();
            let value_int: usize = value.parse().unwrap_throw();

            let mut state = state_handle.deref().clone();
            state.capacity = value_int;
            state_handle.set(state);
        }
    });
    let enc_onclick = Callback::from({
        let state_handle = state_handle.clone();
        move |_| {
            let mut state = state_handle.deref().clone();
            state.encryption = !state.encryption;
            state_handle.set(state);
        }
    });

    let navigator = use_navigator().unwrap();
    let form_onsubmit = Callback::from({
        let navigator = navigator.clone();
        let state_handle = state_handle.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            // TODO: pass room options
            // navigator.push(&Route::Room)
        }
    });
    let cancel_onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div class="container
                    position-relative
                    top-50
                    translate-middle-y"
        >
            <h2 class="pb-2">{ "Room settings" }</h2>
            <div class="border
                        border-2
                        rounded
                        p-3"
            >
                <form onsubmit={form_onsubmit}>
                    <div class="mb-3 form-check form-switch">
                        <input
                            class="form-check-input"
                            id="encryptionCheck"
                            type="checkbox"
                            role="switch"
                            disabled=false
                            checked={state_handle.encryption}
                            onclick={enc_onclick}
                        />
                        <label class="form-check-label" for="encryptionCheck">{ "Encryption" }</label>
                    </div>
                    <div class="mb-3">
                        <div class="d-flex
                                    flex-row
                                    align-items-center
                                    ">
                            <input
                                class="form-range"
                                id="capacityRange"
                                name="capacityRange"
                                type="range"
                                oninput={cap_oninput.clone()}
                                min={MIN_CAPACITY.to_string()}
                                max={MAX_CAPACITY.to_string()}
                                value={state_handle.capacity.to_string()}
                            />
                            <input
                                class="form-control
                                       ms-3
                                       w-auto"
                                name="capacityInput"
                                type="number"
                                oninput={cap_oninput}
                                min={MIN_CAPACITY.to_string()}
                                max={MAX_CAPACITY.to_string()}
                                value={state_handle.capacity.to_string()}
                            />
                        </div>
                        <label class="form-label" for="capacityRange">{ "Capacity" }</label>
                    </div>
                    <button
                        type="submit"
                        class="btn
                               btn-primary"
                    >
                        { "Create" }
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
