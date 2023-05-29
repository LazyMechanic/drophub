use gloo::timers::callback::{Interval, Timeout};
use time::{Duration, OffsetDateTime};
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::Element;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yewdux::prelude::*;

use crate::hooks::use_alert_manager;

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
            AlertKind::Info => {
                html! { <i class="bi bi-info-circle me-2"></i> }
            }
            AlertKind::Success => {
                html! { <i class="bi bi-check-circle me-2"></i> }
            }
            AlertKind::Warn => {
                html! { <i class="bi bi-exclamation-triangle me-2"></i> }
            }
            AlertKind::Error => {
                html! { <i class="bi bi-x-circle me-2"></i> }
            }
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

    fn color_class(&self) -> &'static str {
        match self {
            AlertKind::Info => "toast-info",
            AlertKind::Success => "toast-success",
            AlertKind::Warn => "toast-warning",
            AlertKind::Error => "toast-danger",
        }
    }

    fn progress_bar_color_class(&self) -> &'static str {
        match self {
            AlertKind::Info => "bg-info",
            AlertKind::Success => "bg-success",
            AlertKind::Warn => "bg-warning",
            AlertKind::Error => "bg-danger",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct Props {
    id: String,
    kind: AlertKind,
    message: String,
    delay: Duration,
}

#[function_component(Alert)]
fn alert(props: &Props) -> Html {
    let header = {
        let classes = classes!("toast-header", props.kind.color_class());
        let icon = props.kind.icon();
        let text = props.kind.header_text();

        html! {
            <div class={classes}>
                {icon}
                <strong class="me-auto">{text}</strong>
                <button
                    class="btn-close"
                    type="button"
                    data-bs-dismiss="toast"
                    aria-label="Close"
                >
                </button>
            </div>
        }
    };

    let body = {
        html! {
            <div class="toast-body">{props.message.clone()}</div>
        }
    };

    let progress_bar = {
        let classes = classes!(
            "progress-bar",
            "progress-bar-toast",
            props.kind.progress_bar_color_class()
        );
        let style = format!("--dh-toast-delay: {}ms", props.delay.whole_milliseconds());
        html! {
            <div
                class="progress"
                role="progressbar"
                aria-valuemin="0"
                aria-valuemax="100"
                style="height: 10px"
            >
                <div
                    class={classes}
                    {style}
                ></div>
            </div>
        }
    };

    let toast_classes = classes!("toast", "toast-show", "show", props.kind.color_class());
    html! {
        <div
            class={toast_classes}
            id={props.id.clone()}
            role="alert"
            aria-live="assertive"
            aria-atomic="true"
        >
            {header}
            {body}
            {progress_bar}
        </div>
    }
}

#[function_component(AlertContainer)]
pub fn alert_container() -> Html {
    let alert_man = use_alert_manager();
    let alert_container = use_node_ref();

    let alerts_to_display = alert_man
        .alerts()
        .iter()
        .map(|(alert_id, alert_props)| {
            html! {
                <Alert
                    id={alert_id.clone()}
                    kind={alert_props.kind}
                    message={alert_props.message.clone()}
                    delay={alert_props.delay}
                />
            }
        })
        .collect::<Html>();

    use_effect_with_deps(
        move |(_, alert_container, alert_man)| {
            tracing::debug!(alerts = ?alert_man.alerts(), "Update alert container");

            let alert_container = alert_container
                .cast::<Element>()
                .expect_throw("Failed to cast alert container to Element");

            let mut clear_handles = Vec::with_capacity(alert_man.alerts().len());

            for (alert_id, alert_props) in alert_man.alerts() {
                tracing::debug!(?alert_id, ?alert_props, "Show alert");
                let fade_timeout_delay = {
                    let now = OffsetDateTime::now_utc();
                    let timeout_delay = alert_props.delay - (now - alert_props.init_date());
                    match timeout_delay {
                        d if d.is_negative() => Duration::ZERO,
                        d => d,
                    }
                };
                let hide_timeout_delay = fade_timeout_delay + Duration::seconds(1);

                let alert_elem = alert_container
                    .query_selector(&format!("#{alert_id}"))
                    .expect_throw("Failed to get alert")
                    .expect_throw("Alert not found");

                let fade_timeout_handle =
                    Timeout::new(fade_timeout_delay.whole_milliseconds() as u32, move || {
                        tracing::debug!(id = ?alert_elem.id(), "Fade alert");
                        alert_elem
                            .class_list()
                            .replace("toast-show", "toast-fade")
                            .expect_throw("Failed to replace 'toast-show' to 'toast-fade'");
                    })
                    .forget();
                let hide_timeout_handle =
                    Timeout::new(hide_timeout_delay.whole_milliseconds() as u32, {
                        let alert_man = alert_man.clone();
                        let alert_id = alert_id.clone();
                        move || {
                            tracing::debug!(id = ?alert_id, "Hide alert");
                            alert_man.hide_alert(&alert_id)
                        }
                    })
                    .forget();
                let clear_handle = move || {
                    web_sys::Window::clear_timeout_with_handle(
                        &web_sys::window().unwrap_throw(),
                        fade_timeout_handle.as_f64().unwrap_throw() as i32,
                    );
                    web_sys::Window::clear_timeout_with_handle(
                        &web_sys::window().unwrap_throw(),
                        hide_timeout_handle.as_f64().unwrap_throw() as i32,
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
        (alert_man.alerts().len(), alert_container.clone(), alert_man),
    );

    html! {
        <div
            class="toast-container
                   top-0
                   end-0
                   p-3
                   pt-5
                   position-absolute"
            ref={alert_container}
        >
            {alerts_to_display}
        </div>
    }
}
