use drophub::InvitePassword;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{components::Placeholder, store::Store};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
}

#[function_component(InviteList)]
pub fn invite_list(props: &Props) -> Html {
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
        <div class="d-flex flex-column gap-2">
            <div class="fw-bold">{"Invites:"}</div>
            <div class="btn-group-vertical shadow" role="group" aria-label="Invites">
                {invites}
            </div>
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
