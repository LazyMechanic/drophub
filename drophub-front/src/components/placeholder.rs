use std::fmt::Display;

use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props<T>
where
    T: PartialEq,
{
    #[prop_or(true)]
    pub enabled: bool,
    pub content: T,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Placeholder)]
pub fn placeholder<T>(props: &Props<T>) -> Html
where
    T: PartialEq + Display,
{
    if props.enabled {
        let classes = classes!("placeholder", props.classes.clone());
        html! {
            <span
                class={classes}
                style="vertical-align: baseline;"
            >
                {&props.content}
            </span>
        }
    } else {
        html! {
            <>
                {&props.content}
            </>
        }
    }
}
