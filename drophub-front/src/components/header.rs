use tracing::instrument;
use yew::prelude::*;

#[function_component(Header)]
#[instrument]
pub fn header() -> Html {
    html! {
        <nav
            class="navbar
                   navbar-expand-lg
                   bg-dark"
            data-bs-theme="dark"
        >
            <div class="container-fluid">
                <a class="navbar-brand" href="/">
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
                </a>
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
                        <li class="nav-item"><a class="nav-link" href="/">{ "Home" }</a></li>
                        <li class="nav-item"><a class="nav-link" href="#">{ "About" }</a></li>
                        <li class="nav-item"><a class="nav-link" href="https://github.com/LazyMechanic/drophub">{ "Github" }</a></li>
                        <li class="nav-item"><a class="nav-link" href="#">{ "Contact" }</a></li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
