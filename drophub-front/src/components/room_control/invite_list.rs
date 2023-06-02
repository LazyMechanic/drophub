use drophub::InvitePassword;
use yew::prelude::*;

use crate::components::{room_control::MenuState, Placeholder};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
    pub menu_state: MenuState,
    pub invites: Vec<InvitePassword>,
    pub room_cap: usize,
    pub room_len: usize,
}

#[function_component(InviteList)]
pub fn invite_list(props: &Props) -> Html {
    let header = match props.menu_state {
        MenuState::Expanded => html! {
            <div class="fw-bold">
                <i class="bi bi-envelope-check me-2"></i>
                {"Invites"}
            </div>
        },
        MenuState::Minimized => html! {
            <i class="bi bi-envelope-check text-center"></i>
        },
    };

    let invites = {
        let invite_btns = {
            let rest_invites_count = props.room_cap - props.room_len - props.invites.len();
            let no_more_invites = rest_invites_count == 0;
            let invite_btns_count = if no_more_invites {
                1
            } else {
                rest_invites_count
            };

            std::iter::repeat_with(move || {
                html! {
                    <button
                        class="btn
                               btn-dark"
                        type="button"
                        disabled={no_more_invites}
                        // TODO: add onclick event
                    >
                        <i class="bi bi-plus-lg"></i>
                    </button>
                }
            })
            .take(invite_btns_count)
        };

        props
            .invites
            .iter()
            .map(|invite_password| {
                let onclick = Callback::from(|e: MouseEvent| { /* TODO */ });
                let btn_content = match props.menu_state {
                    MenuState::Expanded => html! {
                        <Placeholder<InvitePassword>
                            enabled={props.placeholder}
                            content={invite_password.clone()}
                        />
                    },
                    MenuState::Minimized => html! {
                        <i class="bi bi-envelope"></i>
                    },
                };

                html! {
                    <button
                        class="btn
                               btn-light
                               font-monospace"
                        type="button"
                        data-bs-toggle="modal"
                        data-bs-target="#inviteModal"
                        {onclick}
                    >
                        {btn_content}
                    </button>
                }
            })
            .chain(invite_btns)
            .collect::<Html>()
    };

    html! {
        <div class="d-flex
                    flex-column 
                    gap-2"
        >
            {header}
            <div class="btn-group-vertical shadow" role="group" aria-label="Invites">
                {invites}
            </div>
        </div>
    }
}
