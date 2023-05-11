use gloo::timers::callback::Timeout;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{Element, HtmlButtonElement, HtmlElement, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(true)]
    pub readonly: bool,
    #[prop_or_default]
    pub content: String,
    #[prop_or_default]
    pub description: Option<String>,
}

#[function_component(CopyInput)]
pub fn copy_input(props: &Props) -> Html {
    let input_ref = use_node_ref();
    let icon_ref = use_node_ref();

    let clipboard = use_clipboard();
    let copy_onclick = Callback::from({
        let input_ref = input_ref.clone();
        let icon_ref = icon_ref.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            let icon = icon_ref
                .cast::<Element>()
                .expect_throw("failed to cast icon_ref to Element");
            let input = input_ref
                .cast::<HtmlInputElement>()
                .expect_throw("failed to cast input_ref to HtmlInputElement");

            icon.class_list()
                .replace("bi-clipboard", "bi-check2")
                .expect_throw("failed to replace class 'bi-clipboard' to 'is-check2'");

            clipboard.write_text(input.value());

            Timeout::new(1500, move || {
                icon.class_list()
                    .replace("bi-check2", "bi-clipboard")
                    .expect_throw("failed to replace class 'bi-check2' to 'is-clipboard'");
            })
            .forget();
        }
    });

    let description = match &props.description {
        None => html! { <></> },
        Some(desc) => html! { <span class="input-group-text">{desc}</span> },
    };

    html! {
        <div
            class="input-group"
        >
            {description}
            <input
                class="form-control"
                type="text"
                value={props.content.clone()}
                readonly={props.readonly}
                ref={input_ref}
            />
            <button
                class="btn
                       btn-primary
                       ms-auto"
                type="button"
                onclick={copy_onclick}
            >
                <i
                    class="bi bi-clipboard"
                    ref={icon_ref}
                ></i>
            </button>
        </div>
    }
}
