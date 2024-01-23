use std::collections::HashMap;

use drophub::{ClientRole, PeerId};
use uuid::Uuid;
use web_sys::Element;
use yew::prelude::*;

use crate::{
    components::{room_control::client_modal::ClientModal, Placeholder},
    hooks::use_notify,
    unwrap_notify_ext::UnwrapNotifyExt,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub clients: HashMap<PeerId, ClientRole>,
    pub cur_client: (PeerId, ClientRole),
    pub capacity: usize,
}

#[function_component(ClientList)]
pub fn client_list(props: &Props) -> Html {
    let notify_manager = use_notify();
    let selected_client_handle = use_state(|| (Uuid::new_v4(), ClientRole::Guest));

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
                .toggle("btn-shade")
                .expect_notify(&notify_manager, "Failed to toggle 'btn-shade' class");
            btn.class_list()
                .toggle("btn-accent")
                .expect_notify(&notify_manager, "Failed to toggle 'btn-accent' class");
        }
    });

    let clients = props
        .clients
        .iter()
        .map(|(id, role)| {
            let onclick = Callback::from({
                let selected_client_handle = selected_client_handle.clone();
                let client = (*id, *role);
                move |_| selected_client_handle.set(client)
            });

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
                           btn-shade-10
                           text-start"
                    type="button"
                    data-bs-toggle="modal"
                    data-bs-target="#dh-room-control-client-modal"
                    {onclick}
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
                       btn-shade
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
            <ClientModal
                loading={props.loading}
                selected_client={*selected_client_handle}
                cur_client={props.cur_client}
            />
        </>
    }
}

fn format_short_client_id(client_id: PeerId) -> String {
    let client_id = client_id.to_string();
    format!(
        "{}...{}",
        &client_id[0..8],
        &client_id[client_id.len() - 12..client_id.len()]
    )
}
