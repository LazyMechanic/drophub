use std::{
    collections::{hash_map::Iter, HashMap},
    ops::Deref,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};

use time::{Duration, OffsetDateTime};
use tracing::instrument;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::AlertKind;

#[hook]
pub fn use_alert_manager() -> AlertManager {
    let (s, d) = use_store::<AlertStore>();
    AlertManager {
        store: s,
        dispatch: d,
    }
}

#[derive(Debug, Clone, Default, PartialEq, Store)]
struct AlertStore {
    alerts: HashMap<AlertId, AlertProps>,
}

#[derive(Clone, PartialEq)]
pub struct AlertManager {
    store: Rc<AlertStore>,
    dispatch: Dispatch<AlertStore>,
}

impl AlertManager {
    #[instrument(skip(self))]
    pub fn show_alert(&self, props: AlertProps) {
        let id = Self::next_id();
        self.dispatch
            .reduce_mut(move |store| store.alerts.insert(id, props));
    }

    pub fn alerts(&self) -> &HashMap<AlertId, AlertProps> {
        &self.store.alerts
    }

    #[instrument(skip(self))]
    pub fn hide_alert(&self, alert_id: &str) {
        self.dispatch.reduce_mut(move |s| s.alerts.remove(alert_id))
    }

    fn next_id() -> AlertId {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        format!("alert0{}", COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

pub type AlertId = String;

#[derive(Debug, Clone, PartialEq)]
pub struct AlertProps {
    pub kind: AlertKind,
    pub message: String,
    pub delay: Duration,
    init_date: OffsetDateTime,
}

impl AlertProps {
    pub fn new(kind: AlertKind, message: String, delay: Duration) -> Self {
        Self {
            kind,
            message,
            delay,
            init_date: OffsetDateTime::now_utc(),
        }
    }

    pub fn info<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self::new(AlertKind::Info, message.to_string(), Self::def_delay())
    }

    pub fn success<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self::new(AlertKind::Success, message.to_string(), Self::def_delay())
    }

    pub fn warn<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self::new(AlertKind::Warn, message.to_string(), Self::def_delay())
    }

    pub fn error<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self::new(AlertKind::Error, message.to_string(), Self::def_delay())
    }

    pub fn with_delay(mut self, custom_delay: Duration) -> Self {
        self.delay = custom_delay;
        self
    }

    pub fn init_date(&self) -> OffsetDateTime {
        self.init_date
    }

    fn def_delay() -> Duration {
        Duration::seconds(7)
    }
}
