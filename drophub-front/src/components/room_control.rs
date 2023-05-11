use drophub::{ClientId, InvitePassword, RoomId};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{copy_input::CopyInput, placeholder::Placeholder, qr_code::QrCode},
    store::{ClientRole, Room, Store},
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
}

#[function_component(RoomControl)]
pub fn room_control(props: &Props) -> Html {
    html! {
        // TODO: change color by role
        <div
            class="d-flex
                   flex-column
                   text-bg-secondary
                   h-100
                   p-2
                   gap-2"
        >
            <RoomInfo placeholder={props.placeholder} />
            <ClientList placeholder={props.placeholder} />
            <InviteList placeholder={props.placeholder} />
            <button
                class="btn
                       btn-primary
                       shadow"
                type="button"
            >
                { "Invite" }
            </button>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct RoomInfoProps {
    #[prop_or_default]
    placeholder: bool,
}

#[function_component(RoomInfo)]
fn room_info(props: &RoomInfoProps) -> Html {
    let store = use_store_value::<Store>();
    let room = &store.room;

    html! {
        <div class="d-flex flex-column">
            <GroupHeader content={"Room info:".to_owned()} />
            <ul class="list-group shadow"
            >
                <li class="list-group-item">
                    <div class="fw-bold">{ "Room ID:" }</div>
                    <Placeholder<RoomId>
                        enabled={props.placeholder}
                        content={room.info.room_id}
                        size={7}
                    />
                </li>
                <li class="list-group-item">
                    <div class="fw-bold">{ "Host:" }</div>
                    <span class="font-monospace">
                        <Placeholder<ClientId>
                            enabled={props.placeholder}
                            content={room.info.host_id}
                            size={12}
                        />
                    </span>
                </li>
            </ul>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct ClientListProps {
    #[prop_or_default]
    placeholder: bool,
}

#[function_component(ClientList)]
fn client_list(props: &ClientListProps) -> Html {
    let store = use_store_value::<Store>();
    let room = &store.room;

    let clients = {
        room.info
            .clients
            .iter()
            .map(|client_id| {
                // TODO: highlight all owned files on hover
                let btn_classes = classes!(
                    "btn",
                    "dropdown-toggle",
                    "caret-off",
                    "font-monospace",
                    if client_id == &room.client.id {
                        "btn-primary"
                    } else {
                        "btn-light"
                    }
                );
                let kick_classes = classes!(
                    "dropdown-item",
                    if client_id == &room.client.id {
                        Some("disabled")
                    } else {
                        None
                    }
                );

                html! {
                    <div
                        class="btn-group
                               dropend"
                        role="group"
                    >
                        <button
                            class={btn_classes}
                            type="button"
                            data-bs-toggle="dropdown"
                            data-bs-auto-close="outside"
                            aria-expanded="false"
                            style="padding-left: 1em !important;
                                   padding-right: 1em !important;"
                        >
                            <Placeholder<ClientId>
                                enabled={props.placeholder}
                                content={client_id.clone()}
                                size={12}
                            />
                        </button>
                        <ul class="dropdown-menu">
                            <li>
                                <button
                                    class={kick_classes}
                                    type="button"
                                    // TODO: add onclick event
                                >
                                    { "Kick" }
                                </button>
                            </li>
                        </ul>
                    </div>
                }
            })
            .collect::<Html>()
    };

    html! {
        <div class="d-flex flex-column">
            <GroupHeader content={"Clients:".to_owned()} />
            <div class="btn-group-vertical shadow" role="group" aria-label="Clients">
                {clients}
            </div>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct InviteListProps {
    #[prop_or_default]
    placeholder: bool,
}

#[function_component(InviteList)]
fn invite_list(props: &InviteListProps) -> Html {
    let store = use_store_value::<Store>();
    let room = &store.room;

    let invites = {
        room.info
            .invites
            .iter()
            .map(|invite_password| {
                let onclick = Callback::from(|e: MouseEvent| { /* TODO */ });

                html! {
                    <div
                        class="btn-group
                               dropend"
                        role="group"
                    >
                        <button
                            class="btn
                                   btn-light
                                   text-start
                                   font-monospace"
                            type="button"
                            data-bs-toggle="modal"
                            data-bs-target="#inviteModal"
                            style="padding-left: 1em !important;
                                   padding-right: 1em !important;"
                            {onclick}
                        >
                            <Placeholder<InvitePassword>
                                enabled={props.placeholder}
                                content={invite_password.clone()}
                                size={6}
                            />
                        </button>
                    </div>
                }
            })
            .collect::<Html>()
    };

    html! {
        <div class="d-flex flex-column">
            <GroupHeader content={"Invites:".to_owned()} />
            <div class="btn-group-vertical shadow" role="group" aria-label="Invites">
                {invites}
            </div>
            <InviteModal placeholder={props.placeholder} />
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct InviteModalProps {
    #[prop_or_default]
    placeholder: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct InviteModalState {
    room_id: RoomId,
    invite_password: InvitePassword,
    invite_link: String,
}

#[function_component(InviteModal)]
fn invite_modal(props: &InviteModalProps) -> Html {
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

            InviteModalState {
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

#[derive(Debug, Clone, PartialEq, Properties)]
struct GroupHeaderProps {
    content: String,
}

#[function_component(GroupHeader)]
fn group_header(props: &GroupHeaderProps) -> Html {
    html! {
        <div class="fw-bold
                    mb-1"
        >
            {props.content.clone()}
        </div>
    }
}
