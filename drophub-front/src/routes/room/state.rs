use std::collections::HashMap;

use drophub::{ClientId, FileMeta, InvitePassword, JwtEncoded, RoomInfo, RoomOptions};
use lazy_static::lazy_static;
use uuid::Uuid;

use crate::routes::room::query::Query;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct State {
    pub query: Option<Query>,
    pub client: ClientInfo,
    pub room: RoomInfo,
    pub selected_invite: Option<InvitePassword>,
    pub loading: bool,
}

impl Default for State {
    fn default() -> Self {
        Self::placeholder_host().clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClientInfo {
    pub jwt: JwtEncoded,
    pub id: ClientId,
    pub role: ClientRole,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ClientRole {
    Host,
    Guest,
}

impl State {
    pub fn placeholder(role: ClientRole) -> &'static Self {
        match role {
            ClientRole::Host => Self::placeholder_host(),
            ClientRole::Guest => Self::placeholder_guest(),
        }
    }

    pub fn placeholder_host() -> &'static Self {
        lazy_static! {
            static ref PLACEHOLDER_HOST: State = {
                let client_id = Uuid::new_v4();
                State {
                    query: None,
                    client: ClientInfo {
                        jwt: JwtEncoded {
                            access_token: "".into(),
                            refresh_token: "".into(),
                        },
                        id: client_id,
                        role: ClientRole::Host,
                    },
                    room: RoomInfo {
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
                    selected_invite: None,
                    loading: true,
                }
            };
        }

        &*PLACEHOLDER_HOST
    }

    pub fn placeholder_guest() -> &'static Self {
        lazy_static! {
            static ref PLACEHOLDER_GUEST: State = {
                let client_id = Uuid::new_v4();
                let host_id = Uuid::new_v4();
                State {
                    query: None,
                    client: ClientInfo {
                        jwt: JwtEncoded {
                            access_token: "".into(),
                            refresh_token: "".into(),
                        },
                        id: client_id,
                        role: ClientRole::Host,
                    },
                    room: RoomInfo {
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
                    selected_invite: None,
                    loading: true,
                }
            };
        }

        &*PLACEHOLDER_GUEST
    }
}
