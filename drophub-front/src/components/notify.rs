use gloo::timers::callback::{Interval, Timeout};
use time::{Duration, OffsetDateTime};
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::Element;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yewdux::prelude::*;

use crate::hooks::use_notify;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NotifyKind {
    Info,
    Success,
    Warn,
    Error,
}

impl NotifyKind {
    fn icon(&self) -> Html {
        match self {
            NotifyKind::Info => {
                html! { <i class="bi bi-info-circle me-2"></i> }
            }
            NotifyKind::Success => {
                html! { <i class="bi bi-check-circle me-2"></i> }
            }
            NotifyKind::Warn => {
                html! { <i class="bi bi-exclamation-triangle me-2"></i> }
            }
            NotifyKind::Error => {
                html! { <i class="bi bi-x-circle me-2"></i> }
            }
        }
    }

    fn header_text(&self) -> &'static str {
        match self {
            NotifyKind::Info => "Info",
            NotifyKind::Success => "Success",
            NotifyKind::Warn => "Warning",
            NotifyKind::Error => "Error",
        }
    }

    fn color_class(&self) -> &'static str {
        match self {
            NotifyKind::Info => "notify-info",
            NotifyKind::Success => "notify-success",
            NotifyKind::Warn => "notify-warning",
            NotifyKind::Error => "notify-danger",
        }
    }

    fn progress_bar_color_class(&self) -> &'static str {
        match self {
            NotifyKind::Info => "bg-info",
            NotifyKind::Success => "bg-success",
            NotifyKind::Warn => "bg-warning",
            NotifyKind::Error => "bg-danger",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct Props {
    id: String,
    kind: NotifyKind,
    message: String,
    delay: Duration,
}

#[function_component(Notify)]
fn notify(props: &Props) -> Html {
    let node_ref = use_node_ref();

    let header = {
        let classes = classes!("toast-header", props.kind.color_class());
        let icon = props.kind.icon();
        let text = props.kind.header_text();
        let onclick = Callback::from({
            let node_ref = node_ref.clone();
            move |_| {
                let elem = node_ref
                    .cast::<Element>()
                    .expect_throw("Failed to cast node to 'Element'");
                let ok = elem
                    .class_list()
                    .replace("notify-show", "notify-fade")
                    .expect_throw("Failed to replace 'notify-show' to 'notify-fade'");

                if ok {
                    tracing::debug!(id = ?elem.id(), "Fade notify manually");
                }
            }
        });

        html! {
            <div class={classes}>
                {icon}
                <strong class="me-auto">{text}</strong>
                <button
                    class="btn-close"
                    type="button"
                    {onclick}
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
            "progress-bar-notify",
            props.kind.progress_bar_color_class()
        );
        let style = format!("--dh-notify-delay: {}ms", props.delay.whole_milliseconds());
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

    let toast_classes = classes!("toast", "notify-show", "show", props.kind.color_class());
    html! {
        <div
            class={toast_classes}
            id={props.id.clone()}
            role="alert"
            aria-live="assertive"
            aria-atomic="true"
            ref={node_ref}
        >
            {header}
            {body}
            {progress_bar}
        </div>
    }
}

#[function_component(NotifyContainer)]
pub fn notify_container() -> Html {
    let notify_manager = use_notify();
    let container_ref = use_node_ref();

    let notifies_to_display = notify_manager
        .notifies()
        .iter()
        .map(|(notify_id, notify_props)| {
            html! {
                <Notify
                    id={notify_id.clone()}
                    kind={notify_props.kind}
                    message={notify_props.message.clone()}
                    delay={notify_props.delay}
                />
            }
        })
        .collect::<Html>();

    use_effect_with_deps(
        move |(_, container_ref, notify_manager)| {
            tracing::debug!(notifies = ?notify_manager.notifies(), "Update notify container");

            let container_elem = container_ref
                .cast::<Element>()
                .expect_throw("Failed to cast notify container to 'Element'");

            let mut clear_handles = Vec::with_capacity(notify_manager.notifies().len());

            for (notify_id, notify_props) in notify_manager.notifies() {
                tracing::debug!(?notify_id, ?notify_props, "Show notify");
                let fade_timeout_delay = {
                    let now = OffsetDateTime::now_utc();
                    let timeout_delay = notify_props.delay - (now - notify_props.init_date());
                    match timeout_delay {
                        d if d.is_negative() => Duration::ZERO,
                        d => d,
                    }
                };
                let hide_timeout_delay = fade_timeout_delay + Duration::seconds(1);

                let notify_elem = container_elem
                    .query_selector(&format!("#{notify_id}"))
                    .expect_throw("Failed to get notify")
                    .expect_throw("Notify not found");

                let fade_timeout_handle =
                    Timeout::new(fade_timeout_delay.whole_milliseconds() as u32, move || {
                        let ok = notify_elem
                            .class_list()
                            .replace("notify-show", "notify-fade")
                            .expect_throw("Failed to replace 'notify-show' to 'notify-fade'");

                        if ok {
                            tracing::debug!(id = ?notify_elem.id(), "Fade notify");
                        }
                    })
                    .forget();
                let hide_timeout_handle =
                    Timeout::new(hide_timeout_delay.whole_milliseconds() as u32, {
                        let notify_manager = notify_manager.clone();
                        let notify_id = notify_id.clone();
                        move || {
                            tracing::debug!(id = ?notify_id, "Hide notify");
                            notify_manager.hide_notify(&notify_id)
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
        (
            notify_manager.notifies().len(),
            container_ref.clone(),
            notify_manager,
        ),
    );

    html! {
        <div
            class="toast-container
                   top-0
                   end-0
                   p-3
                   pt-5
                   position-absolute"
            ref={container_ref}
        >
            {notifies_to_display}
        </div>
    }
}
