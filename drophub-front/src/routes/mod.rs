pub mod connect_room;
pub mod create_room;
pub mod home;
pub mod not_found;
pub mod room;

use drophub::{RoomId, RoomOptions};
use yew::prelude::*;
use yew_router::prelude::*;

use self::{
    connect_room::ConnectRoom, create_room::CreateRoom, home::Home, not_found::NotFound, room::Room,
};

/// App routes
#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/room/create")]
    CreateRoom,
    #[at("/room/connect")]
    ConnectRoom,
    #[at("/room")]
    Room,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::CreateRoom => html! { <CreateRoom/> },
        Route::ConnectRoom => html! { <ConnectRoom/> },
        Route::Room => html! { <Room /> },
        Route::NotFound => html! { <NotFound/> },
    }
}
