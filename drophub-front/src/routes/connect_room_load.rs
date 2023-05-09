use drophub::{InvitePassword, RoomId};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::file_card::{FileCard, FileUpload};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub room_id: RoomId,
    pub invite_password: InvitePassword,
}

#[function_component(ConnectRoomLoad)]
pub fn connect_room_load(props: &Props) -> Html {
    // TODO: send api request
    html! {
        <div class="d-flex
                    flex-row
                    h-100
                    placeholder-glow"
        >
            <div
                class="d-flex
                       flex-column
                       text-bg-secondary
                       h-100
                       p-2"
                style="width: 300px;"
            >
                <ul class="list-group
                           pb-2"
                >
                    <li class="list-group-item">
                        <div class="fw-bold">{ "Room ID" }</div>
                        <span class="placeholder
                                     col-7"
                        ></span>
                    </li>
                    <li class="list-group-item">
                        <div class="fw-bold">{ "Admin" }</div>
                        <span class="placeholder
                                     col-8"
                        ></span>
                    </li>
                </ul>
                <ol class="list-group
                           list-group-numbered
                           pb-2"
                >
                    <li class="list-group-item
                               list-group-item-action
                               active"
                    >
                        <span class="placeholder
                                     col-7"
                        ></span>
                    </li>
                    <li class="list-group-item
                               list-group-item-action"
                    >
                        <span class="placeholder
                                     col-6"
                        ></span>
                    </li>
                    <li class="list-group-item
                               list-group-item-action"
                    >
                        <span class="placeholder
                                     col-8"
                        ></span>
                    </li>
                    <li class="list-group-item
                               list-group-item-action"
                    >
                        <span class="placeholder
                                     col-10"
                        ></span>
                    </li>
                </ol>
                <button
                    class="btn
                           btn-primary"
                    type="button"
                >
                    { "Invite" }
                </button>
            </div>
            <div class="container
                        ms-1
                        me-1"
            >
                <div class="row row-cols-auto">
                    <FileCard placeholder={true} />
                    <FileCard placeholder={true} />
                    <FileCard placeholder={true} />
                    <FileCard placeholder={true} />
                    <FileCard placeholder={true} />
                    <FileUpload onclick={Callback::noop()} />
                </div>
            </div>
        </div>
    }
}
