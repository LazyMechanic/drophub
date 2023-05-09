use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, HtmlSelectElement};
use yew::prelude::*;

#[hook]
pub fn use_form_validation() -> NodeRef {
    let form = use_node_ref();
    use_effect_with_deps(
        {
            let form = form.clone();
            move |_| {
                let cb = Callback::from({
                    let form = form.clone();
                    move |e: Event| {
                        if let Some(select) = form.cast::<HtmlSelectElement>() {
                            if !select.check_validity() {
                                e.prevent_default();
                                e.stop_propagation();
                            }
                        }

                        if let Some(form) = form.cast::<HtmlElement>() {
                            form.class_list().add_1("was-validated").unwrap_throw();
                        }
                    }
                });

                let listener = Closure::<dyn Fn(Event)>::wrap(Box::new(move |e: Event| cb.emit(e)));

                if let Some(element) = form.cast::<HtmlElement>() {
                    element
                        .add_event_listener_with_callback(
                            "submit",
                            listener.as_ref().dyn_ref().unwrap_throw(),
                        )
                        .unwrap_throw();
                }

                move || drop(listener)
            }
        },
        form.clone(),
    );

    form
}
