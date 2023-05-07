use drophub::RoomId;
use tracing::instrument;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub room_id: RoomId,
}

#[function_component(Room)]
#[instrument]
pub fn room(props: &Props) -> Html {
    html! { <h1>{"todo"}</h1> }
}
