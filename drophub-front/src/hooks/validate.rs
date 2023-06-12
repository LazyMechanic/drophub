use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlElement, HtmlSelectElement};
use yew::prelude::*;

use crate::{hooks::use_notify, unwrap_notify_ext::UnwrapNotifyExt};

#[hook]
pub fn use_form_validation() -> NodeRef {
    let form = use_node_ref();
    let notify_manager = use_notify();
    use_effect_with_deps(
        move |form| {
            let cb = Callback::from({
                let form = form.clone();
                let notify_manager = notify_manager.clone();
                move |e: Event| {
                    if let Some(select) = form.cast::<HtmlSelectElement>() {
                        if !select.check_validity() {
                            e.prevent_default();
                            e.stop_propagation();
                        }
                    }

                    if let Some(form) = form.cast::<HtmlElement>() {
                        form.class_list()
                            .add_1("was-validated")
                            .unwrap_notify(&notify_manager);
                    }
                }
            });

            let listener = Closure::<dyn Fn(Event)>::wrap(Box::new(move |e: Event| cb.emit(e)));

            if let Some(element) = form.cast::<HtmlElement>() {
                element
                    .add_event_listener_with_callback(
                        "submit",
                        listener.as_ref().dyn_ref().unwrap_notify(&notify_manager),
                    )
                    .unwrap_notify(&notify_manager);
            }

            move || drop(listener)
        },
        form.clone(),
    );

    form
}
