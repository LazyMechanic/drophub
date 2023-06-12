use std::{pin::pin, rc::Rc, time::Duration};

use futures::{FutureExt, StreamExt};
use yew::{
    platform::{
        spawn_local,
        time::{interval, sleep},
    },
    prelude::*,
};
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{Footer, FullScreenLoading, FullScreenNotify, Header, NotifyContainer},
    config::Config,
    error::{Error, ShareError},
    hooks::{use_rpc_storage, RpcStorage},
    routes::{switch, Route},
};

#[function_component(App)]
pub fn app() -> Html {
    let (rpc, rpc_dispatch) = use_rpc_storage();
    let connect_handle = use_async_with_options(
        connect_to_server(rpc_dispatch),
        UseAsyncOptions::enable_auto(),
    );

    let main_content = {
        html! {
            <>
                if connect_handle.loading {
                    <FullScreenLoading />
                }
                if let Some(err) = &connect_handle.error {
                    <FullScreenNotify<String> content={format!("Failed to load page: {err}")} />
                }
                if let Some(_) = &connect_handle.data {
                    <Switch<Route> render={switch} />
                }
            </>
        }
    };

    html! {
        <BrowserRouter>
            <div class="d-flex
                        flex-column
                        h-100
                        w-100"
            >
                <header><Header /></header>
                <main class="flex-grow-1">
                    {main_content}
                    <NotifyContainer />
                </main>
                <footer><Footer /></footer>
            </div>
        </BrowserRouter>
    }
}

async fn connect_to_server(rpc_dispatch: Dispatch<RpcStorage>) -> Result<(), ShareError> {
    let cfg = Config::from_env()?;
    let rpc_client: jsonrpsee::core::client::Client =
        jsonrpsee::wasm_client::WasmClientBuilder::default()
            .build(cfg.api_server_url)
            .await
            .map_err(Error::from)?;

    let mut connect_timeout = pin!(sleep(cfg.init_timeout).fuse());
    let mut interval = pin!(interval(Duration::from_millis(250)).fuse());
    loop {
        futures::select_biased! {
            _ = rpc_client.on_disconnect().fuse() => return Err(Error::Other(anyhow::anyhow!("Disconnect from API server")).into()),
            _ = &mut connect_timeout => return Err(Error::Other(anyhow::anyhow!("Connection to API server timed out")).into()),
            _ = interval.next() => {},
        }

        if rpc_client.is_connected() {
            break;
        }
    }

    rpc_dispatch.reduce_mut(|s| s.rpc_client = Some(Rc::new(rpc_client)));

    Ok(())
}
