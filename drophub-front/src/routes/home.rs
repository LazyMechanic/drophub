use tracing::instrument;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(Home)]
#[instrument]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let connect_room_btn = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::ConnectRoom));
        html! {
            <button
                class="btn
                       btn-secondary
                       text-center
                       text-nowrap
                       fs-2
                       position-relative"
                id="dhBtnConnectRoom"
                type="button"
                disabled=false
                {onclick}
            >
                { "Connect to room" }
            </button>
        }
    };

    let create_room_btn = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::CreateRoom));
        html! {
            <button
                class="btn
                       btn-primary
                       text-center
                       text-nowrap
                       fs-2
                       position-relative"
                id="dhBtnCreateRoom"
                type="button"
                disabled=false
                {onclick}
            >
                { "Create new room" }
            </button>
        }
    };

    html! {
        <div class="container h-100">
            <div class="row h-100">
                <div
                    class="col-12
                           col-lg-6
                           d-flex
                           flex-column
                           flex-lg-row
                           justify-content-end
                           justify-content-lg-right
                           align-items-center
                           mb-3
                           mb-lg-0"
                >
                    {connect_room_btn}
                </div>
                <div
                    class="col-12
                           col-lg-6
                           d-flex
                           flex-column
                           flex-lg-row
                           justify-content-start
                           justify-content-lg-left
                           align-items-center
                           mt-3
                           mt-lg-0"
                >
                    {create_room_btn}
                </div>
            </div>
        </div>
    }
}
