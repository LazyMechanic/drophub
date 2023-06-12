use std::fmt::Display;

use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props<T: PartialEq> {
    pub content: T,
}

#[function_component(FullScreenNotify)]
pub fn full_screen_notify<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Display,
{
    html! {
        <div
            class="d-flex
                   justify-content-center
                   text-center
                   align-items-center
                   h-100"
        >
            {&props.content}
        </div>
    }
}
