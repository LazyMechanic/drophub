pub mod home;
pub mod not_found;
pub mod room;

use drophub::RoomId;
use yew::prelude::*;
use yew_router::prelude::*;

use self::{home::Home, not_found::NotFound, room::Room};

/// App routes
#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/room/:room_id")]
    Room { room_id: RoomId },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Room { room_id } => html! { <Room room_id={room_id} /> },
        Route::NotFound => html! { <NotFound/> },
    }
}
