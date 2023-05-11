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
                   w-100
                   bg-dark"
        >
            <ul class="nav nav-underline">
                <li class="nav-item"><Link<Route> classes="nav-link text-secondary" to={Route::Home}>{ "Home" }</Link<Route>></li>
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
