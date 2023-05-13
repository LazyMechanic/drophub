use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav
            class="navbar
                   navbar-expand-lg
                   shadow
                   bg-dark"
            data-bs-theme="dark"
        >
            <div class="container-fluid">

                <Link<Route> classes="navbar-brand fs-4" to={Route::Home}>
                    <img
                        src="https://img.icons8.com/?size=512&id=EVJQEyN2gkSr&format=png"
                        alt="Logo"
                        width="30"
                        height="24"
                        class="d-inline-block
                               align-text-top"
                        style="margin-right: 0.5em;"
                    />
                    { "Drophub" }
                </Link<Route>>
                <button
                    class="navbar-toggler"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#navbarSupportedContent"
                    aria-controls="navbarSupportedContent"
                    aria-expanded="false"
                    aria-label="Toggle navigation"
                >
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        <li class="nav-item"><Link<Route> classes="nav-link" to={Route::Home}>{ "Home" }</Link<Route>></li>
                        <li class="nav-item"><a class="nav-link" href="#">{ "About" }</a></li>
                        <li class="nav-item"><a class="nav-link" href="https://github.com/LazyMechanic/drophub">{ "Github" }</a></li>
                        <li class="nav-item"><a class="nav-link" href="#">{ "Contact" }</a></li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
