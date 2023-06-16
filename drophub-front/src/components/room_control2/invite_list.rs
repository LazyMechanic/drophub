use drophub::InvitePassword;
use web_sys::Element;
use yew::prelude::*;

use crate::{components::Placeholder, hooks::use_notify, unwrap_notify_ext::UnwrapNotifyExt};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub invites: Vec<InvitePassword>,
    pub capacity: usize,
    pub clients_count: usize,
}

#[function_component(InviteList)]
pub fn invite_list(props: &Props) -> Html {
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

    let invites = props
        .invites
        .iter()
        .map(|invite_password| {
            html! {
                <button
                    class="btn
                           btn-collapse-item-body
                           text-start"
                    type="button"
                    data-bs-toggle="modal"
                    data-bs-target="#dh-room-control-invite-modal"
                >
                    <i class="bi
                              bi-envelope"
                    ></i>
                    <span class="dh-room-control-hidden
                                 font-monospace
                                 ms-2
                                 d-inline-block"
                    >
                        <Placeholder<InvitePassword>
                            enabled={props.loading}
                            content={invite_password.clone()}
                        />
                    </span>
                </button>
            }
        })
        .chain(
            std::iter::repeat_with(|| {
                let rest_invites_count = props.capacity - props.clients_count - props.invites.len();
                let no_more_invites = rest_invites_count == 0;
                html! {
                    <button
                        class="btn
                               btn-collapse-item-body
                               text-center"
                        type="button"
                        disabled={no_more_invites}
                    >
                        <i class="bi
                                  bi-plus-lg
                                  dh-room-control-icon"
                        ></i>
                    </button>
                }
            })
            .take(1),
        )
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
                data-bs-target="#dh-room-control-invite-collapse"
                aria-expanded="false"
                aria-controls="dh-room-control-invite-collapse"
                onclick={collapse_btn_onclick}
                ref={btn_node_ref}
            >
                <i class="bi
                          bi-envelope-check"
                ></i>
                <span class="d-inline-block
                             ms-2
                             me-auto
                             dh-room-control-hidden"
                >
                    {"Invites "}
                    <Placeholder<String>
                        enabled={props.loading}
                        content={format!("{} / {}", props.invites.len(), props.capacity - props.clients_count)}
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
                id="dh-room-control-invite-collapse"
            >
                <div
                    class="btn-group-vertical
                           w-100"
                    role="group"
                >
                    {invites}
                </div>
            </div>
        </>
    }
}
