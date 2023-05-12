use drophub::{InvitePassword, RoomId};
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{CopyInput, QrCode},
    store::Store,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    room_id: RoomId,
    invite_password: InvitePassword,
    invite_link: String,
}

#[function_component(InviteModal)]
pub fn invite_modal(props: &Props) -> Html {
    let store = use_store_value::<Store>();
    let state_handle = use_state_eq({
        let store = store.clone();
        move || {
            let room_id = store.room.info.room_id;
            let invite_password = store
                .selected_invite
                .clone()
                .unwrap_or_else(|| "placeholder".to_owned());
            let invite_link = {
                let win = web_sys::window().expect_throw("failed to get Window");
                let base_url = win.location().origin().unwrap_throw();
                format_invite_link(&base_url, room_id, &invite_password)
            };

            State {
                room_id,
                invite_password,
                invite_link,
            }
        }
    });

    html! {
        <div
            class="modal
                   modal-dialog-centered
                   fade"
            id="inviteModal"
            tabindex="-1"
            aria-labelledby="inviteModalLabel"
            aria-hidden="true"
            style="display: none;"
        >
            <div class="modal-dialog">
                <div class="modal-content
                            text-bg-light"
                >
                    <div class="modal-header">
                        <h1 class="modal-title fs-4" id="inviteModalLabel">
                            {"Invite"}
                        </h1>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <p >{"Use one of the options below to connect:"}</p>
                        <div class="d-flex
                                    flex-column
                                    p-2
                                    gap-3"
                        >
                            <div>
                                <h6>{"1. Scan QR code"}</h6>
                                <QrCode<String>
                                    value={state_handle.invite_link.clone()}
                                    size={300}
                                />
                            </div>
                            <div>
                                <h6>{"2. Follow the link"}</h6>
                                <CopyInput content={state_handle.invite_link.clone()} />
                            </div>
                            <div>
                                <h6>{"3. Enter credentials manually"}</h6>
                                <div class="d-flex
                                            flex-row
                                            gap-2"
                                >
                                    <div>
                                        <span>{"Room ID"}</span>
                                        <CopyInput content={state_handle.room_id.to_string()} />
                                    </div>
                                    <div>
                                        <span>{"Invite password"}</span>
                                        <CopyInput content={state_handle.invite_password.clone()} />
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="modal-footer">
                        <button
                            class="btn
                                   btn-danger"
                            type="button"
                            data-bs-dismiss="modal"
                            // TODO: add revoke op
                        >
                            {"Revoke"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn format_invite_link(base_url: &str, room_id: RoomId, invite_password: &str) -> String {
    format!("{}/room/connect/{}/{}", base_url, room_id, invite_password)
}
