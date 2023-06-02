use drophub::{InvitePassword, RoomId};
use yew::prelude::*;

use crate::{
    components::{CopyInput, QrCode},
    hooks::use_notify,
    unwrap_notify_ext::UnwrapNotifyExt,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
    pub room_id: RoomId,
    pub selected_invite: InvitePassword,
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    invite_link: String,
}

#[function_component(InviteModal)]
pub fn invite_modal(props: &Props) -> Html {
    let notify_manager = use_notify();

    let state_handle = use_state_eq({
        let notify_manager = notify_manager.clone();
        let props = props.clone();
        move || {
            let room_id = props.room_id;
            let invite_password = props.selected_invite;
            let invite_link = {
                let win = web_sys::window().expect_notify(&notify_manager, "Failed to get Window");
                let base_url = win
                    .location()
                    .origin()
                    .expect_notify(&notify_manager, "Failed to get origin");
                format_invite_link(&base_url, room_id, &invite_password)
            };

            State { invite_link }
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
                                            gap-3"
                                >
                                    <div>
                                        <span>{"Room ID"}</span>
                                        <CopyInput content={props.room_id.to_string()} />
                                    </div>
                                    <div>
                                        <span>{"Invite password"}</span>
                                        <CopyInput content={props.selected_invite.clone()} />
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
                            // TODO: add onclick event
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
    format!("{base_url}/room?action=connect&room_id={room_id}&invite_password={invite_password}")
}
