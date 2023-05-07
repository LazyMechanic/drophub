use tracing::instrument;
use yew::prelude::*;

#[function_component(NotFound)]
#[instrument]
pub fn not_found() -> Html {
    html! {
        <div class="container-fluid
                    position-relative
                    top-50
                    translate-middle-y">
            <img
                class="w-50
                       rounded
                       mx-auto
                       d-block"
                src="https://cdn.shazoo.ru/327997_gyd5en79dd_were_sorry_soooo_sorry.jpg"
                alt="We're Sorry" />
            <h2 class="text-center">{"Page not found"}</h2>
        </div>
    }
}
