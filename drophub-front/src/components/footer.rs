use tracing::instrument;
use yew::prelude::*;

#[function_component(Footer)]
#[instrument]
pub fn footer() -> Html {
    html! {
        <div
            class="d-flex
                   flex-column
                   justify-content-center
                   text-center
                   align-items-center
                   w-100
                   bg-light"
        >
            <ul class="nav nav-underline">
                <li class="nav-item"><a class="nav-link text-secondary" href="/">{ "Home" }</a></li>
                <li class="nav-item"><a class="nav-link text-secondary" href="#">{ "About" }</a></li>
                <li class="nav-item"><a class="nav-link text-secondary" href="https://github.com/LazyMechanic/drophub">{ "Github" }</a></li>
                <li class="nav-item"><a class="nav-link text-secondary" href="#">{ "Contact" }</a></li>
            </ul>
            <p
                class="text-secondary
                       fs-6"
                style="margin: 10px;"
            >
                { "\u{A9}2023 LazyMechanic | All Rights Reserved" }
            </p>
        </div>
    }
}
