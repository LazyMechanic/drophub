use std::{
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
};

use drophub::{ClientId, FileMeta, InvitePassword, JwtEncoded, RoomInfo, RoomOptions};
use lazy_static::lazy_static;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use yewdux::prelude::*;

use crate::{components::alert::AlertKind, rpc, rpc::RpcRequestTx};

static COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Store)]
pub struct Store {
    rpc_tx: RpcRequestTx,
    pub alerts: Vec<AlertProps>,
    pub room: Room,
    pub selected_invite: Option<InvitePassword>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            rpc_tx: rpc::channel().0,
            alerts: vec![],
            room: Room::placeholder_host().clone(),
            selected_invite: None,
        }
    }
}

impl Store {
    pub fn new(rpc_tx: RpcRequestTx) -> Self {
        Self {
            alerts: vec![],
            rpc_tx,
            room: Room::placeholder_host().clone(),
            selected_invite: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Room {
    pub client: Client,
    pub info: RoomInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Client {
    pub jwt: JwtEncoded,
    pub id: ClientId,
    pub role: ClientRole,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ClientRole {
    Host,
    Guest,
}

impl Room {
    pub fn placeholder(role: ClientRole) -> &'static Self {
        match role {
            ClientRole::Host => Self::placeholder_host(),
            ClientRole::Guest => Self::placeholder_guest(),
        }
    }

    pub fn placeholder_host() -> &'static Self {
        lazy_static! {
            static ref ROOM_PLACEHOLDER_HOST: Room = {
                let client_id = Uuid::new_v4();
                Room {
                    client: Client {
                        jwt: JwtEncoded {
                            access_token: "".into(),
                            refresh_token: "".into(),
                        },
                        id: client_id,
                        role: ClientRole::Host,
                    },
                    info: RoomInfo {
                        room_id: 123456,
                        host_id: client_id,
                        files: {
                            let mut f = HashMap::new();
                            f.insert(
                                73532627,
                                FileMeta {
                                    name: "text.txt".into(),
                                    size: 256,
                                    checksum: 73532627,
                                },
                            );
                            f.insert(
                                12512517,
                                FileMeta {
                                    name: "movie.mp4".into(),
                                    size: 521425,
                                    checksum: 12512517,
                                },
                            );
                            f.insert(
                                68854343,
                                FileMeta {
                                    name: "music.mp3".into(),
                                    size: 33521,
                                    checksum: 68854343,
                                },
                            );
                            f.insert(
                                157899765,
                                FileMeta {
                                    name: "word.doc".into(),
                                    size: 14512,
                                    checksum: 157899765,
                                },
                            );
                            f.insert(
                                678534342,
                                FileMeta {
                                    name: "image.png".into(),
                                    size: 21512,
                                    checksum: 678534342,
                                },
                            );
                            f
                        },
                        clients: vec![client_id, Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()],
                        invites: vec![
                            "a16DqGr0".into(),
                            "h52hj5wf".into(),
                            "oug19b23".into(),
                            "11jie8fd".into(),
                        ],
                        options: RoomOptions {
                            encryption: false,
                            capacity: 10,
                        },
                    },
                }
            };
        }

        &*ROOM_PLACEHOLDER_HOST
    }

    pub fn placeholder_guest() -> &'static Self {
        lazy_static! {
            static ref ROOM_PLACEHOLDER_GUEST: Room = {
                let client_id = Uuid::new_v4();
                let host_id = Uuid::new_v4();
                Room {
                    client: Client {
                        jwt: JwtEncoded {
                            access_token: "".into(),
                            refresh_token: "".into(),
                        },
                        id: client_id,
                        role: ClientRole::Host,
                    },
                    info: RoomInfo {
                        room_id: 123456,
                        host_id: host_id,
                        files: {
                            let mut f = HashMap::new();
                            f.insert(
                                73532627,
                                FileMeta {
                                    name: "text.txt".into(),
                                    size: 256,
                                    checksum: 73532627,
                                },
                            );
                            f.insert(
                                12512517,
                                FileMeta {
                                    name: "movie.mp4".into(),
                                    size: 521425,
                                    checksum: 12512517,
                                },
                            );
                            f.insert(
                                68854343,
                                FileMeta {
                                    name: "music.mp3".into(),
                                    size: 33521,
                                    checksum: 68854343,
                                },
                            );
                            f.insert(
                                157899765,
                                FileMeta {
                                    name: "word.doc".into(),
                                    size: 14512,
                                    checksum: 157899765,
                                },
                            );
                            f.insert(
                                678534342,
                                FileMeta {
                                    name: "image.png".into(),
                                    size: 21512,
                                    checksum: 678534342,
                                },
                            );
                            f
                        },
                        clients: vec![client_id, host_id, Uuid::new_v4(), Uuid::new_v4()],
                        invites: vec![
                            "a16DqGr0".into(),
                            "h52hj5wf".into(),
                            "oug19b23".into(),
                            "11jie8fd".into(),
                        ],
                        options: RoomOptions {
                            encryption: false,
                            capacity: 10,
                        },
                    },
                }
            };
        }

        &*ROOM_PLACEHOLDER_GUEST
    }
}

impl PartialEq for Store {
    fn eq(&self, other: &Self) -> bool {
        self.alerts == other.alerts && self.room == self.room
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
