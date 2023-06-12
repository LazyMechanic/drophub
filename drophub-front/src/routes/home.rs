use yew::prelude::*;

use crate::components::{ConnectRoomForm, CreateRoomForm};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container-fluid
                    h-100
                    p-3
                    bg-body-secondary"
        >
            <div
                class="bg-body
                       border
                       border-0
                       rounded
                       shadow
                       mx-auto"
                style="max-width: 540px;"
            >
                <ul
                    class="nav
                           nav-tabs
                           nav-justified"
                    id="dh-home-tab"
                    role="tablist"
                >
                    <li class="nav-item" role="presentation">
                        <button
                            class="nav-link
                                   active
                                   dh-home-tab-btn
                                   text-dark"
                            id="dh-home-create-tab"
                            data-bs-toggle="tab"
                            data-bs-target="#dh-home-create-tab-pane"
                            type="button"
                            role="tab"
                            aria-controls="dh-home-create-tab-pane"
                            aria-selected="true"
                        >
                            {"Create room"}
                        </button>
                    </li>
                    <li class="nav-item" role="presentation">
                        <button
                            class="nav-link
                                   dh-home-tab-btn
                                   text-dark"
                            id="dh-home-connect-tab"
                            data-bs-toggle="tab"
                            data-bs-target="#dh-home-connect-tab-pane"
                            type="button"
                            role="tab"
                            aria-controls="dh-home-connect-tab-pane"
                            aria-selected="false"
                        >
                            {"Connect to room"}
                        </button>
                    </li>
                </ul>
                <div
                    class="tab-content
                           border-start
                           border-end
                           border-bottom
                           border-1
                           rounded-bottom
                           bg-body" 
                    id="dh-home-tab-content"
                >
                    <div
                        class="tab-pane
                               fade
                               show
                               active
                               p-3"
                        id="dh-home-create-tab-pane"
                        role="tabpanel"
                        aria-labelledby="profile-tab"
                        tabindex="0"
                    >
                        <CreateRoomForm />
                    </div>
                    <div
                        class="tab-pane
                               fade
                               p-3"
                        id="dh-home-connect-tab-pane"
                        role="tabpanel"
                        aria-labelledby="home-tab"
                        tabindex="0"
                    >
                        <ConnectRoomForm />
                    </div>
                </div>
            </div>
        </div>
    }
}
