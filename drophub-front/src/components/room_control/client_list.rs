use drophub::ClientId;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{components::Placeholder, store::Store};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
}

#[function_component(ClientList)]
pub fn client_list(props: &Props) -> Html {
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
        <div class="d-flex flex-column gap-2">
            <div class="fw-bold">{"Clients:"}</div>
            <div class="btn-group-vertical shadow" role="group" aria-label="Clients">
                {clients}
            </div>
        </div>
    }
}
