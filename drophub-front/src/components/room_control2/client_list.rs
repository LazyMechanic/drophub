use std::collections::HashMap;

use drophub::ClientId;
use yew::prelude::*;

use crate::{
    components::Placeholder, routes::room::state::ClientRole, unwrap_notify_ext::UnwrapNotifyExt,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub clients: HashMap<ClientId, ClientRole>,
    pub cur_client: (ClientId, ClientRole),
    pub capacity: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    host: ClientId,
}

#[function_component(ClientList)]
pub fn client_list(props: &Props) -> Html {
    let clients = props
        .clients
        .iter()
        .map(|(id, role)| {
            // TODO: highlight all owned files on hover
            let btn_classes = classes!(
                "btn",
                "dropdown-toggle",
                "caret-off",
                "font-monospace",
                if *role == ClientRole::Host {
                    "btn-primary"
                } else {
                    "btn-secondary"
                }
            );
            let kick_classes = classes!(
                "dropdown-item",
                if props.cur_client.1 == ClientRole::Host && *id != props.cur_client.0 {
                    None // enabled
                } else {
                    Some("disabled")
                }
            );
            let icon_classes = classes! {
                "bi",
                if *id == props.cur_client.0 {
                    "bi-person-fill"
                } else {
                    "bi-person"
                },
                "dh-room-control-icon",
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
                        <i class={icon_classes}></i>
                        <span class="dh-room-control-text ms-1">
                            <Placeholder<String>
                                enabled={props.loading}
                                content={format_short_client_id(*id)}
                            />
                        </span>
                    </button>
                    <ul class="dropdown-menu">
                        <li><h6 class="dropdown-header">{id}</h6></li>
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
        .collect::<Html>();

    html! {
        <div class="d-flex
                    flex-column
                    gap-2"
        >
            <div class="fw-bold">
                <i class="bi
                          bi-diagram-3
                          dh-room-control-icon"
                ></i>
                <span class="dh-room-control-text ms-1">
                    {"Clients "}
                    <Placeholder<String>
                        enabled={props.loading}
                        content={format!("{} / {}", props.clients.len(), props.capacity)}
                    />
                </span>
            </div>
            <div
                class="btn-group-vertical
                       shadow
                       border
                       border-0
                       rounded"
                role="group"
                aria-label="Clients"
            >
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
