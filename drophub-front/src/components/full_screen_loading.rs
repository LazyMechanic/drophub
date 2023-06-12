use yew::prelude::*;

#[function_component(FullScreenLoading)]
pub fn full_screen_loading() -> Html {
    html! {
        <div
            class="d-flex
                   flex-column
                   justify-content-center
                   text-center
                   align-items-center
                   h-100"
        >
            <div class="spinner-border pb-3" role="status"></div>
            <h5>{"Loading..."}</h5>
        </div>
    }
}
