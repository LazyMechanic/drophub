pub mod connect_room;
pub mod connect_room_load;
pub mod create_room;
pub mod create_room_load;
pub mod home;
pub mod not_found;
pub mod room;

use drophub::{InvitePassword, RoomId};
use yew::prelude::*;
use yew_router::prelude::*;

use self::{
    connect_room::ConnectRoom, connect_room_load::ConnectRoomLoad, create_room::CreateRoom,
    create_room_load::CreateRoomLoad, home::Home, not_found::NotFound, room::Room,
};

/// App routes
#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/room/create")]
    CreateRoom,
    #[at("/room/create/action")]
    CreateRoomLoad,
    #[at("/room/connect")]
    ConnectRoom,
    #[at("/room/connect/:room_id/:invite_password")]
    ConnectRoomLoad {
        room_id: RoomId,
        invite_password: InvitePassword,
    },
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
        Route::CreateRoomLoad => html! { <CreateRoomLoad /> },
        Route::ConnectRoom => html! { <ConnectRoom/> },
        Route::ConnectRoomLoad {
            room_id,
            invite_password,
        } => html! { <ConnectRoomLoad {room_id} {invite_password} /> },
        Route::Room => html! { <Room /> },
        Route::NotFound => html! { <NotFound/> },
    }
}
