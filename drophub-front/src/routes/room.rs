use drophub::{RoomId, RoomOptions};
use tracing::instrument;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component(Room)]
#[instrument]
pub fn room() -> Html {
    // let create_room_handle = use_async({
    //     let state_handle = state_handle.clone();
    //     async move {
    //         let mut rx = ctx_handle
    //             .rpc_tx
    //             .sub(RpcSubscribeRequest::CreateRoom(RoomOptions {
    //                 encryption: state_handle.encryption,
    //                 capacity: state_handle.capacity,
    //             }))?;
    //
    //         let jwt = rx.try_recv().await?;
    //         let room_info = ;
    //
    //         Ok::<_, Error>(())
    //     }
    // });

    html! { <h1>{"todo"}</h1> }
}
