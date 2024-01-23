use drophub::{PeerId, RoomId, RoomOptions};
use yew::prelude::*;

use crate::components::Placeholder;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    pub room_id: RoomId,
    pub room_opts: RoomOptions,
    pub host: PeerId,
}

#[function_component(RoomInfoModal)]
pub fn room_info_modal(props: &Props) -> Html {
    html! {
        <div
            class="modal
                   modal-dialog-centered
                   fade"
            id="dh-room-control-room-info-modal"
            tabindex="-1"
            aria-labelledby="dh-room-control-room-info-modal-label"
            aria-hidden="true"
            style="display: none;"
        >
            <div class="modal-dialog">
                <div class="modal-content
                            bg-shade"
                >
                    <div class="modal-header">
                        <h1 class="modal-title fs-4" id="dh-room-control-room-info-modal-label">
                            {"Room info"}
                        </h1>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <table class="table table-bordered">
                            <tbody>
                                <tr>
                                    <th scope="row">{"Room ID"}</th>
                                    <td>
                                        <Placeholder<RoomId>
                                            enabled={props.loading}
                                            content={props.room_id}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <th scope="row">{"Host ID"}</th>
                                    <td>
                                        <Placeholder<PeerId>
                                            enabled={props.loading}
                                            content={props.host}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <th scope="row">{"Capacity"}</th>
                                    <td>
                                        <Placeholder<usize>
                                            enabled={props.loading}
                                            content={props.room_opts.capacity}
                                        />
                                    </td>
                                </tr>
                                <tr>
                                    <th scope="row">{"Encryption"}</th>
                                    <td>
                                        <Placeholder<bool>
                                            enabled={props.loading}
                                            content={props.room_opts.encryption}
                                        />
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                    <div class="modal-footer">
                        <button
                            class="btn
                                   btn-danger"
                            type="button"
                            data-bs-dismiss="modal"
                            // TODO: add onclick event
                        >
                            {"Disconnect"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
