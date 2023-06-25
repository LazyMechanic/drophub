use drophub::{ClientId, ClientRole};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub selected_client: (ClientId, ClientRole),
    pub cur_client: (ClientId, ClientRole),
}

#[function_component(ClientModal)]
pub fn client_modal(props: &Props) -> Html {
    let is_kick_enabled =
        props.cur_client.1 == ClientRole::Host && props.cur_client.0 != props.selected_client.0;

    html! {
        <div
            class="modal
                   modal-dialog-centered
                   fade"
            id="dh-room-control-client-modal"
            tabindex="-1"
            aria-labelledby="dh-room-control-client-modal-label"
            aria-hidden="true"
            style="display: none;"
        >
            <div class="modal-dialog">
                <div class="modal-content
                            bg-shade"
                >
                    <div class="modal-header">
                        <h1 class="modal-title fs-4" id="dh-room-control-client-modal-label">
                            {"Client info"}
                        </h1>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        {"TODO: id, role, media list, etc."}
                    </div>
                    <div class="modal-footer">
                        <button
                            class="btn
                                   btn-danger"
                            type="button"
                            data-bs-dismiss="modal"
                            disabled={!is_kick_enabled}
                            // TODO: add onclick event
                        >
                            {"Kick"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
