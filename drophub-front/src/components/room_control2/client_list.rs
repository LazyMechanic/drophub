use std::collections::HashMap;

use drophub::ClientId;
use web_sys::Element;
use yew::prelude::*;

use crate::{
    components::Placeholder, hooks::use_notify, routes::room::state::ClientRole,
    unwrap_notify_ext::UnwrapNotifyExt,
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
    let notify_manager = use_notify();

    let icon_node_ref = use_node_ref();
    let btn_node_ref = use_node_ref();
    let collapse_btn_onclick = Callback::from({
        let icon_node_ref = icon_node_ref.clone();
        let btn_node_ref = btn_node_ref.clone();
        move |_| {
            let icon = icon_node_ref
                .cast::<Element>()
                .expect_notify(&notify_manager, "Failed to cast 'NodeRef' to 'Element'");
            icon.class_list()
                .toggle("show")
                .expect_notify(&notify_manager, "Failed to toggle 'show' class");

            let btn = btn_node_ref
                .cast::<Element>()
                .expect_notify(&notify_manager, "Failed to cast 'NodeRef' to 'Element'");
            btn.class_list()
                .toggle("btn-collapse")
                .expect_notify(&notify_manager, "Failed to toggle 'btn-collapse' class");
        }
    });

    let clients = props
        .clients
        .iter()
        .map(|(id, role)| {
            // TODO: highlight all owned files on hover
            let icon_classes = classes! {
                "bi",
                if *id == props.cur_client.0 {
                    match role {
                        ClientRole::Host => "bi-person-fill-gear",
                        ClientRole::Guest => "bi-person-fill",
                    }
                } else {
                    match role {
                        ClientRole::Host => "bi-person-gear",
                        ClientRole::Guest => "bi-person",
                    }
                },
                "dh-room-control-icon",
            };

            html! {
                <button
                    class="btn
                           btn-collapse-item-body
                           text-start"
                    type="button"
                    // TODO: open modal
                >
                    <i class={icon_classes}></i>
                    <span class="dh-room-control-hidden
                                 ms-2
                                 font-monospace
                                 d-inline-block"
                    >
                        <Placeholder<String>
                            enabled={props.loading}
                            content={format_short_client_id(*id)}
                        />
                    </span>
                </button>
            }
        })
        .collect::<Html>();

    html! {
        <>
            <button
                class="btn
                       btn-body
                       d-flex
                       flex-row 
                       position-relative"
                type="button"
                data-bs-toggle="collapse"
                data-bs-target="#dh-room-control-client-collapse"
                aria-expanded="false"
                aria-controls="dh-room-control-client-collapse"
                onclick={collapse_btn_onclick}
                ref={btn_node_ref}
            >
                <i class="bi
                          bi-people"
                ></i>
                <span class="d-inline-block
                             ms-2
                             me-auto
                             dh-room-control-hidden"
                >
                    {"Clients "}
                    <Placeholder<String>
                        enabled={props.loading}
                        content={format!("{} / {}", props.clients.len(), props.capacity)}
                    />
                    <i
                        class="bi
                               bi-chevron-right
                               collapse-icon"
                        ref={icon_node_ref}
                    ></i>
                </span>
            </button>
            <div
                class="collapse"
                id="dh-room-control-client-collapse"
            >
                <div
                    class="btn-group-vertical
                           w-100"
                    role="group"
                >
                    {clients}
                </div>
            </div>
        </>
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
