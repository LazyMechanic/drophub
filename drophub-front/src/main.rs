mod app;
mod components;
mod config;
mod error;
mod hooks;
mod routes;
mod rpc;
mod unwrap_notify_ext;
mod validate;

use app::App;

fn main() {
    init_logging();
    run_client();
}

fn init_logging() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
}

fn run_client() {
    yew::Renderer::<App>::new().render();
}
