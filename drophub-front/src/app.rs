use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::{footer::Footer, header::Header},
    routes::{switch, Route},
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div
                class="d-flex
                       flex-column
                       h-100
                       w-100"
            >
                <header><Header /></header>
                <main class="flex-grow-1"><Switch<Route> render={switch} /></main>
                <footer><Footer /></footer>
            </div>
        </BrowserRouter>
    }
}
