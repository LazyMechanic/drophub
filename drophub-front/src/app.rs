use wasm_bindgen::UnwrapThrowExt;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::{
    components::{footer::Footer, header::Header},
    config::Config,
    ctx::Context,
    routes::{switch, Route},
    rpc,
    rpc::RpcRequestTx,
};

#[function_component(App)]
pub fn app() -> Html {
    let cfg = Config::from_env().unwrap_throw();
    let (rpc_tx, rpc_rx) = rpc::channel();
    spawn_local(rpc::run(cfg.clone(), rpc_rx));

    let ctx = use_reducer(|| Context::new(rpc_tx));

    html! {
        <ContextProvider<UseReducerHandle<Context>> context={ctx}>
            <BrowserRouter>
                <div class="d-flex
                            flex-column
                            h-100
                            w-100"
                >
                    <header><Header /></header>
                    <main class="flex-grow-1 mt-3 mb-3"><Switch<Route> render={switch} /></main>
                    <footer><Footer /></footer>
                </div>
            </BrowserRouter>
        </ContextProvider<UseReducerHandle<Context>>>
    }
}
