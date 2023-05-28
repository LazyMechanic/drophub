use drophub::ClientId;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{room_control::MenuState, Placeholder},
    hooks::use_room_store_value,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
    pub menu_state: MenuState,
}

#[function_component(ClientList)]
pub fn client_list(props: &Props) -> Html {
    let store = use_room_store_value();
    let room = &store.room;

    let header = match props.menu_state {
        MenuState::Expanded => html! {
            <div class="fw-bold">
                <i class="bi bi-diagram-3 me-2"></i>
                {"Clients"}
            </div>
        },
        MenuState::Minimized => html! {
            <i class="bi bi-diagram-3 text-center"></i>
        },
    };

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

                let btn_content = match props.menu_state {
                    MenuState::Expanded => html! {
                        <Placeholder<ClientId>
                            enabled={props.placeholder}
                            content={client_id.clone()}
                        />
                    },
                    MenuState::Minimized => html! {
                        <i class="bi bi-person"></i>
                    },
                };

                html! {
                    <div class="btn-group
                                dropend"
                    >
                        <button
                            class={btn_classes}
                            type="button"
                            data-bs-toggle="dropdown"
                            data-bs-auto-close="outside"
                            aria-expanded="false"
                        >
                            {btn_content}
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
        <div class="d-flex
                    flex-column 
                    gap-2"
        >
            {header}
            <div class="btn-group-vertical shadow" role="group" aria-label="Clients">
                {clients}
            </div>
        </div>
    }
}
