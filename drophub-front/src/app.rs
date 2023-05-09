use wasm_bindgen::UnwrapThrowExt;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{alert::AlertContainer, footer::Footer, header::Header, icons::Icons},
    config::Config,
    routes::{switch, Route},
    rpc,
    store::Store,
};

#[function_component(App)]
pub fn app() -> Html {
    let cfg = Config::from_env().unwrap_throw();
    let (rpc_tx, rpc_rx) = rpc::channel();
    spawn_local(rpc::run(cfg, rpc_rx));

    // Init store
    use_effect_with_deps(
        move |_| {
            Dispatch::<Store>::new().set(Store::new(rpc_tx));
            || {}
        },
        (),
    );

    html! {
        <BrowserRouter>
            <Icons />
            <div class="d-flex
                        flex-column
                        h-100
                        w-100"
            >
                <header><Header /></header>
                <main class="flex-grow-1">
                    <AlertContainer />
                    <Switch<Route> render={switch} />
                </main>
                <footer><Footer /></footer>
            </div>
        </BrowserRouter>
    }
}
