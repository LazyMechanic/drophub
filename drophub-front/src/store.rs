use std::sync::atomic::{AtomicU64, Ordering};

use time::{Duration, OffsetDateTime};
use yewdux::prelude::*;

use crate::components::alert::AlertKind;

static COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Default, Store)]
pub struct Store {
    pub alerts: Vec<AlertProps>,
}

impl PartialEq for Store {
    fn eq(&self, other: &Self) -> bool {
        self.alerts.len().eq(&other.alerts.len())
    }
}

pub fn add_alert(dispatch: &Dispatch<Store>, alert: AlertProps) {
    dispatch.reduce_mut(move |store| store.alerts.push(alert))
}

#[derive(Debug, Clone, PartialEq)]
pub struct AlertProps {
    id: String,
    pub kind: AlertKind,
    pub message: String,
    pub delay: Duration,
    pub init_date: OffsetDateTime,
}

impl AlertProps {
    pub fn new(kind: AlertKind, message: String, delay: Duration) -> Self {
        Self {
            id: Self::next_id(),
            kind,
            message,
            delay,
            init_date: OffsetDateTime::now_utc(),
        }
    }

    pub fn info(message: String, delay: Duration) -> Self {
        Self::new(AlertKind::Info, message, delay)
    }

    pub fn success(message: String, delay: Duration) -> Self {
        Self::new(AlertKind::Success, message, delay)
    }

    pub fn warn(message: String, delay: Duration) -> Self {
        Self::new(AlertKind::Warn, message, delay)
    }

    pub fn error(message: String, delay: Duration) -> Self {
        Self::new(AlertKind::Error, message, delay)
    }

    pub fn id(&self) -> &str {
        &self.id[1..]
    }

    pub fn id_selector(&self) -> &str {
        &self.id
    }

    fn next_id() -> String {
        format!("#alert{}", COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}
