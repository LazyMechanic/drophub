use wasm_bindgen::UnwrapThrowExt;
use yew::{platform::spawn_local, prelude::*};
use yew_hooks::use_effect_once;
use yew_router::prelude::*;

use crate::{
    components::{Footer, Header, NotifyContainer},
    config::Config,
    hooks,
    routes::{switch, Route},
    rpc,
};

#[function_component(App)]
pub fn app() -> Html {
    // TODO: fullscreen placeholder with error
    let cfg = Config::from_env().unwrap_throw();
    let (rpc_tx, rpc_rx) = rpc::channel();
    spawn_local(rpc::run(cfg, rpc_rx));

    // Init store
    use_effect_once(move || {
        hooks::init_rpc(rpc_tx);
        || {}
    });

    html! {
        <BrowserRouter>
            <div class="d-flex
                        flex-column
                        h-100
                        w-100"
            >
                <header><Header /></header>
                <main class="flex-grow-1">
                    <Switch<Route> render={switch} />
                    <NotifyContainer />
                </main>
                <footer><Footer /></footer>
            </div>
        </BrowserRouter>
    }
}
