use drophub::{ClientId, RoomId, RoomOptions};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub placeholder: bool,
    pub room_id: RoomId,
    pub room_opts: RoomOptions,
    pub host: ClientId,
}

#[function_component(RoomInfoModal)]
pub fn room_info_modal(props: &Props) -> Html {
    html! {
        <div
            class="modal
                   modal-dialog-centered
                   fade"
            id="roomInfoModal"
            tabindex="-1"
            aria-labelledby="roomInfoModalLabel"
            aria-hidden="true"
            style="display: none;"
        >
            <div class="modal-dialog">
                <div class="modal-content
                            text-bg-light"
                >
                    <div class="modal-header">
                        <h1 class="modal-title fs-4" id="roomInfoModalLabel">
                            {"Room info"}
                        </h1>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <table class="table table-bordered">
                            <tbody>
                                <tr>
                                    <th scope="row">{"Room ID"}</th>
                                    <td>{props.room_id}</td>
                                </tr>
                                <tr>
                                    <th scope="row">{"Host ID"}</th>
                                    <td>{props.host}</td>
                                </tr>
                                <tr>
                                    <th scope="row">{"Encryption"}</th>
                                    <td>{props.room_opts.encryption}</td>
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
