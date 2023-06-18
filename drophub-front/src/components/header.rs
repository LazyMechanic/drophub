use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::components::Link;

use crate::{
    hooks::{use_display_mode, use_notify, DisplayMode},
    routes::Route,
    unwrap_notify_ext::UnwrapNotifyExt,
};

#[function_component(Header)]
pub fn header() -> Html {
    let notify_manager = use_notify();

    let display_mode_btn_node_ref = use_node_ref();
    let display_mode_handle = use_display_mode();

    use_effect_once({
        let notify_manager = notify_manager.clone();
        let display_mode_handle = display_mode_handle.clone();
        let display_mode_btn_node_ref = display_mode_btn_node_ref.clone();
        move || {
            let window = web_sys::window().expect_notify(&notify_manager, "Failed to get 'Window'");
            let init_display_mode = match *display_mode_handle {
                Some(v) => v,
                None => {
                    let is_dark = window
                        .match_media("(prefers-color-scheme: dark)")
                        .expect_notify(&notify_manager, "Failed to match media")
                        .expect_notify(&notify_manager, "'MediaQueryList' is missing")
                        .matches();

                    if is_dark {
                        DisplayMode::Dark
                    } else {
                        DisplayMode::Light
                    }
                }
            };

            display_mode_handle.set(init_display_mode);
            tracing::debug!(?init_display_mode, "Display mode init");
            let root_elem = window
                .document()
                .expect_notify(&notify_manager, "Failed to get 'Document'")
                .document_element()
                .expect_notify(&notify_manager, "Failed to get document element");

            root_elem
                .attributes()
                .get_named_item("data-bs-theme")
                .expect_notify(&notify_manager, "'data-bs-theme' attribute is missing")
                .set_value(init_display_mode.as_str());

            || {}
        }
    });

    let display_mode_onclick = Callback::from({
        let display_mode_btn_node_ref = display_mode_btn_node_ref.clone();
        let display_mode_handle = display_mode_handle.clone();
        move |_| {
            let display_mode_btn_elem = display_mode_btn_node_ref
                .cast::<HtmlInputElement>()
                .expect_notify(
                    &notify_manager,
                    "Failed to cast 'NodeRef' to 'HtmlInputElement'",
                );

            let new_display_mode = if display_mode_btn_elem.checked() {
                DisplayMode::Dark
            } else {
                DisplayMode::Light
            };

            display_mode_handle.set(new_display_mode);
            tracing::debug!(?new_display_mode, "Display mode changed");
            let root_elem = web_sys::window()
                .expect_notify(&notify_manager, "Failed to get 'Window'")
                .document()
                .expect_notify(&notify_manager, "Failed to get 'Document'")
                .document_element()
                .expect_notify(&notify_manager, "Failed to get document element");

            root_elem
                .attributes()
                .get_named_item("data-bs-theme")
                .expect_notify(&notify_manager, "'data-bs-theme' attribute is missing")
                .set_value(new_display_mode.as_str());
        }
    });

    html! {
        <nav
            class="navbar
                   navbar-expand-lg
                   shadow"
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
                    <div class="form-check form-switch">
                        <input
                            class="form-check-input"
                            type="checkbox"
                            role="switch"
                            id="display-mode"
                            ref={display_mode_btn_node_ref}
                            onclick={display_mode_onclick}
                            checked={display_mode_handle.as_ref().map(|m| m.is_dark()).unwrap_or_default()}
                        />
                        <label
                            class="form-check-label"
                            for="display-mode"
                        >
                            <i class="bi bi-moon-stars"></i>
                        </label>
                    </div>
                </div>
            </div>
        </nav>
    }
}
