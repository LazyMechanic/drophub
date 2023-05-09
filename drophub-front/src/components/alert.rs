use futures::StreamExt;
use gloo::timers::callback::Timeout;
use time::{Duration, OffsetDateTime};
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::Node;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::Store;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AlertKind {
    Info,
    Success,
    Warn,
    Error,
}

impl AlertKind {
    fn icon(&self) -> Html {
        match self {
            AlertKind::Info => html! {
                <div class="icon-link">
                    <svg
                        class="bi
                               flex-shrink-0 
                               me-2"
                        role="img"
                        aria-label="Info:"
                        style="height: 1.25em;
                               width: 1.25em;"
                    >
                        <use href="#symbol-info"/>
                    </svg>
                </div>
            },
            AlertKind::Success => html! {
                <div class="icon-link">
                    <svg
                        class="bi
                               flex-shrink-0 
                               me-2"
                        role="img"
                        aria-label="Success:"
                        style="height: 1.25em;
                               width: 1.25em;"
                    >
                        <use href="#symbol-ok"/>
                    </svg>
                </div>
            },
            AlertKind::Warn => html! {
                <div class="icon-link">
                    <svg
                        class="bi
                               flex-shrink-0 
                               me-2"
                        role="img"
                        aria-label="Warning:"
                        style="height: 1.25em;
                               width: 1.25em;"
                    >
                        <use href="#symbol-warn"/>
                    </svg>
                </div>
            },
            AlertKind::Error => html! {
                <div class="icon-link">
                    <svg
                        class="bi
                               flex-shrink-0 
                               me-2"
                        role="img"
                        aria-label="Error:"
                        style="height: 1.25em;
                               width: 1.25em;"
                    >
                        <use href="#symbol-warn"/>
                    </svg>
                </div>
            },
        }
    }

    fn header_text(&self) -> &'static str {
        match self {
            AlertKind::Info => "Info",
            AlertKind::Success => "Success",
            AlertKind::Warn => "Warning",
            AlertKind::Error => "Error",
        }
    }

    fn colors_class(&self) -> &'static str {
        match self {
            AlertKind::Info => "toast-info",
            AlertKind::Success => "toast-success",
            AlertKind::Warn => "toast-warning",
            AlertKind::Error => "toast-danger",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub id: String,
    pub kind: AlertKind,
    pub message: String,
}

#[function_component(Alert)]
pub fn alert(props: &Props) -> Html {
    let icon = props.kind.icon();
    let header_text = props.kind.header_text();
    let colors_class = props.kind.colors_class();
    html! {
        <div
            class={format!("toast show fade {}", colors_class)}
            id={props.id.clone()}
            role="alert"
            aria-live="assertive"
            aria-atomic="true"
        >
            <div class={format!("toast-header {}", colors_class)}>
                {icon}
                <strong class="me-auto">{ header_text }</strong>
                <button
                    class="btn-close"
                    type="button"
                    data-bs-dismiss="toast"
                    aria-label="Close"
                >
                </button>
            </div>
            <div class="toast-body">{props.message.clone()}</div>
        </div>
    }
}

#[function_component(AlertContainer)]
pub fn alert_container() -> Html {
    let (store, dispatch) = use_store::<Store>();
    use_effect_with_deps(
        move |(alerts, dispatch)| {
            let alert_container = gloo::utils::document()
                .query_selector("#alertContainer")
                .expect_throw("failed to select alertContainer")
                .expect_throw("alertContainer not found");

            let mut clear_handles = Vec::with_capacity(alerts.len());

            for (i, alert_props) in alerts.iter().enumerate() {
                let timeout_delay = {
                    let now = OffsetDateTime::now_utc();
                    let timeout_delay = alert_props.delay - (now - alert_props.init_date);
                    match timeout_delay {
                        d if d.is_negative() => Duration::ZERO,
                        d => d,
                    }
                };
                // TODO: add fade on remove
                let timeout_handle = Timeout::new(timeout_delay.whole_milliseconds() as u32, {
                    let alert_container = alert_container.clone();
                    let alert_props = alert_props.clone();
                    let dispatch = dispatch.clone();
                    move || {
                        tracing::debug!(
                            "Timeout reached, remove alert {:?}",
                            alert_props.id_selector()
                        );
                        dispatch.reduce_mut(|store| store.alerts.remove(i));
                        let alert = gloo::utils::document()
                            .query_selector(alert_props.id_selector())
                            .expect_throw("failed to select alert")
                            .expect_throw("alert not found");
                        alert_container
                            .remove_child(&alert)
                            .expect_throw("failed to remove alert");
                    }
                })
                .forget();
                let clear_handle = move || {
                    web_sys::Window::clear_timeout_with_handle(
                        &web_sys::window().unwrap_throw(),
                        timeout_handle.as_f64().unwrap_throw() as i32,
                    );
                };

                clear_handles.push(Box::new(clear_handle) as Box<dyn FnOnce()>)
            }

            move || {
                for h in clear_handles {
                    h();
                }
            }
        },
        (store.alerts.clone(), dispatch),
    );

    let alerts = store
        .alerts
        .iter()
        .map(|alert_props| {
            html! {
                <Alert
                    id={alert_props.id().to_owned()}
                    kind={alert_props.kind}
                    message={alert_props.message.clone()}
                />
            }
        })
        .collect::<Html>();

    html! {
        <div
            class="toast-container
                   top-0
                   end-0
                   p-3
                   pt-5
                   position-absolute"
            id="alertContainer"
        >
            {alerts}
        </div>
    }
}
