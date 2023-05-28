use drophub::{ClientId, RoomId, RoomOptions};
use time::OffsetDateTime;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::hooks::use_room_store_value;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    pub room_id: RoomId,
    pub host_id: ClientId,
    pub room_opts: RoomOptions,
}

#[function_component(RoomInfoModal)]
pub fn room_info_modal(props: &Props) -> Html {
    let store = use_room_store_value();
    let state_handle = use_state_eq({
        let store = store.clone();
        move || State {
            room_id: store.room.info.room_id,
            host_id: store.room.info.host_id,
            room_opts: store.room.info.options.clone(),
        }
    });

    html! {
        <div
            class="modal
                   modal-dialog-centered
                   fade"
            id="roomInfoModal"
            tabindex="-1"
            aria-labelledby="roomInfoModalLabel"
            aria-hidden="true"
            style="display: none;"
        >
            <div class="modal-dialog">
                <div class="modal-content
                            text-bg-light"
                >
                    <div class="modal-header">
                        <h1 class="modal-title fs-4" id="roomInfoModalLabel">
                            {"Room info"}
                        </h1>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <table class="table table-bordered">
                            <tbody>
                                <tr>
                                    <th scope="row">{"Room ID"}</th>
                                    <td>{state_handle.room_id}</td>
                                </tr>
                                <tr>
                                    <th scope="row">{"Host ID"}</th>
                                    <td>{state_handle.host_id}</td>
                                </tr>
                                <tr>
                                    <th scope="row">{"Encryption"}</th>
                                    <td>{state_handle.room_opts.encryption}</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    <div class="modal-footer">
                        <button
                            class="btn
                                   btn-danger"
                            type="button"
                            data-bs-dismiss="modal"
                            // TODO: add onclick event
                        >
                            {"Disconnect"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
