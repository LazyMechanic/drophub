use tracing::instrument;
use yew::prelude::*;

#[function_component(Home)]
#[instrument]
pub fn home() -> Html {
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
                    <button
                        class="btn
                               btn-secondary
                               text-center
                               text-nowrap
                               fs-2
                               position-relative"
                        id="dh-btn-connect-room"
                        type="button"
                        disabled=false
                    >
                        { "Connect to room" }
                    </button>
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
                    <button
                        class="btn
                               btn-primary
                               text-center
                               text-nowrap
                               fs-2
                               position-relative"
                        id="dh-btn-create-room"
                        type="button"
                        disabled=false
                    >
                        { "Create new room" }
                    </button>
                </div>
            </div>
        </div>
    }
}
