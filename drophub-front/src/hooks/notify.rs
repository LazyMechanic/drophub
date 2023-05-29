use std::{
    collections::HashMap,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};

use time::{Duration, OffsetDateTime};
use tracing::instrument;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::NotifyKind;

#[hook]
pub fn use_notify() -> NotifyManager {
    let (s, d) = use_store::<NotifyStore>();
    NotifyManager {
        store: s,
        dispatch: d,
    }
}

#[derive(Debug, Clone, Default, PartialEq, Store)]
struct NotifyStore {
    notifies: HashMap<NotifyId, NotifyProps>,
}

#[derive(Clone, PartialEq)]
pub struct NotifyManager {
    store: Rc<NotifyStore>,
    dispatch: Dispatch<NotifyStore>,
}

impl NotifyManager {
    #[instrument(skip(self))]
    pub fn show_notify(&self, props: NotifyProps) {
        let id = Self::next_id();
        self.dispatch
            .reduce_mut(move |store| store.notifies.insert(id, props));
    }

    pub fn notifies(&self) -> &HashMap<NotifyId, NotifyProps> {
        &self.store.notifies
    }

    #[instrument(skip(self))]
    pub fn hide_notify(&self, notify_id: &str) {
        self.dispatch
            .reduce_mut(move |s| s.notifies.remove(notify_id))
    }

    fn next_id() -> NotifyId {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        format!("notify0{}", COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

pub type NotifyId = String;

#[derive(Debug, Clone, PartialEq)]
pub struct NotifyProps {
    pub kind: NotifyKind,
    pub message: String,
    pub delay: Duration,
    init_date: OffsetDateTime,
}

impl NotifyProps {
    pub fn new(kind: NotifyKind, message: String, delay: Duration) -> Self {
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
        Self::new(NotifyKind::Info, message.to_string(), Self::def_delay())
    }

    pub fn success<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self::new(NotifyKind::Success, message.to_string(), Self::def_delay())
    }

    pub fn warn<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self::new(NotifyKind::Warn, message.to_string(), Self::def_delay())
    }

    pub fn error<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self::new(NotifyKind::Error, message.to_string(), Self::def_delay())
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
