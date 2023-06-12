pub mod connect_room;
pub mod create_room;
pub mod home;
pub mod not_found;
pub mod room;

use drophub::{InvitePassword, RoomId};
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
    #[at("/room")]
    Room,
    #[at("/room/connect/:room_id/:invite_password")]
    ConnectRoom {
        room_id: RoomId,
        invite_password: InvitePassword,
    },
    #[at("/room/create")]
    CreateRoom,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Room => html! { <Room /> },
        Route::ConnectRoom {
            room_id,
            invite_password,
        } => html! { <ConnectRoom {room_id} {invite_password} /> },
        Route::CreateRoom => html! { <CreateRoom /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
