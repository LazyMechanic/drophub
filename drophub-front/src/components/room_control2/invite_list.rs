use drophub::InvitePassword;
use yew::prelude::*;

use crate::{components::Placeholder, unwrap_notify_ext::UnwrapNotifyExt};

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
    let invites = props
        .invites
        .iter()
        .map(|invite_password| {
            html! {
                <button
                    class="btn
                           btn-secondary
                           font-monospace"
                    type="button"
                    data-bs-toggle="modal"
                    data-bs-target="#dh-room-control-invite-modal"
                >
                    <i class="bi
                              bi-envelope
                              dh-room-control-icon"
                    ></i>
                    <span class="dh-room-control-text ms-1">
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
                               btn-primary
                               font-monospace"
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
        <div class="d-flex
                    flex-column
                    gap-2"
        >
            <div class="fw-bold">
                <i class="bi
                          bi-envelope-check
                          dh-room-control-icon"
                ></i>
                <span class="dh-room-control-text ms-1">
                    {"Invites "}
                    <Placeholder<String>
                        enabled={props.loading}
                        content={format!("{} / {}", props.invites.len(), props.capacity - props.clients_count)}
                    />
                </span>
            </div>
            <div
                class="btn-group-vertical
                       border
                       border-0
                       rounded
                       shadow"
                role="group"
                aria-label="Invites"
            >
                {invites}
            </div>
        </div>
    }
}
