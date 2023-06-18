use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div
            class="d-flex
                   flex-column
                   justify-content-center
                   text-center
                   align-items-center
                   bg-shade"
        >
            <ul class="nav nav-underline">
                <li class="nav-item"><Link<Route> classes="nav-link link-contrast-shade" to={Route::Home}>{ "Home" }</Link<Route>></li>
                <li class="nav-item"><a class="nav-link link-contrast-shade" href="#">{ "About" }</a></li>
                <li class="nav-item"><a class="nav-link link-contrast-shade" href="https://github.com/LazyMechanic/drophub">{ "Github" }</a></li>
                <li class="nav-item"><a class="nav-link link-contrast-shade" href="#">{ "Contact" }</a></li>
            </ul>
            <p
                class="fs-6"
                style="margin: 10px;"
            >
                { "\u{A9}2023 LazyMechanic | All Rights Reserved" }
            </p>
        </div>
    }
}
