use drophub::ClientId;
use yew::prelude::*;

use crate::components::{room_control::MenuState, Placeholder};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
    pub menu_state: MenuState,
    pub clients: Vec<ClientId>,
    pub host: ClientId,
    pub cur_client: ClientId,
}

#[function_component(ClientList)]
pub fn client_list(props: &Props) -> Html {
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
        props
            .clients
            .iter()
            .map(|client_id| {
                // TODO: highlight all owned files on hover
                let btn_classes = classes!(
                    "btn",
                    "dropdown-toggle",
                    "caret-off",
                    "font-monospace",
                    if *client_id == props.host {
                        "btn-primary"
                    } else {
                        "btn-light"
                    }
                );
                let kick_classes = classes!(
                    "dropdown-item",
                    if props.cur_client == props.host && client_id != &props.cur_client {
                        None // enabled
                    } else {
                        Some("disabled")
                    }
                );

                let btn_content = match props.menu_state {
                    MenuState::Expanded => html! {
                        <Placeholder<String>
                            enabled={props.placeholder}
                            content={format_short_client_id(client_id.clone())}
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
                            <li><h6 class="dropdown-header">{client_id}</h6></li>
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

fn format_short_client_id(client_id: ClientId) -> String {
    let client_id = client_id.to_string();
    format!(
        "{}...{}",
        &client_id[0..8],
        &client_id[client_id.len() - 12..client_id.len()]
    )
}
