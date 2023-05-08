mod app;
mod components;
mod config;
mod ctx;
mod error;
mod routes;
mod rpc;

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
