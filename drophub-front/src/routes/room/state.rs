use drophub::{
    AccessTokenEncoded, ClientId, ClientRole, EntityMeta, FileMeta, RoomInfo, RoomOptions, TextMeta,
};
use indexmap::IndexMap;
use lazy_static::lazy_static;
use uuid::Uuid;

use crate::routes::room::query::Query;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct State {
    pub client: ClientInfo,
    pub room: RoomInfo,
    pub loading: bool,
    pub query: Option<Query>,
}

impl Default for State {
    fn default() -> Self {
        Self::placeholder_host().clone()
    }
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
                    client: ClientInfo {
                        token: "".into(),
                        id: client_id,
                        role: ClientRole::Host,
                    },
                    room: RoomInfo {
                        id: 123456,
                        host: client_id,
                        entities: {
                            let mut f = IndexMap::new();
                            f.insert(
                                73532627,
                                EntityMeta::File(FileMeta {
                                    name: "text.txt".into(),
                                    size: 256,
                                }),
                            );
                            f.insert(
                                12512517,
                                EntityMeta::File(FileMeta {
                                    name: "movie.mp4".into(),
                                    size: 521425,
                                }),
                            );
                            f.insert(
                                68854343,
                                EntityMeta::File(FileMeta {
                                    name: "music.mp3".into(),
                                    size: 33521,
                                }),
                            );
                            f.insert(
                                157899765,
                                EntityMeta::File(FileMeta {
                                    name: "word.doc".into(),
                                    size: 14512,
                                }),
                            );
                            f.insert(
                                678534342,
                                EntityMeta::File(FileMeta {
                                    name: "image.png".into(),
                                    size: 21512,
                                }),
                            );
                            f.insert(
                                678534342,
                                EntityMeta::Text(TextMeta {
                                    name: "text".into(),
                                }),
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
                    loading: true,
                    query: None,
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
                    client: ClientInfo {
                        token: "".into(),
                        id: client_id,
                        role: ClientRole::Host,
                    },
                    room: RoomInfo {
                        id: 123456,
                        host: host_id,
                        entities: {
                            let mut f = IndexMap::new();
                            f.insert(
                                73532627,
                                EntityMeta::File(FileMeta {
                                    name: "text.txt".into(),
                                    size: 256,
                                }),
                            );
                            f.insert(
                                12512517,
                                EntityMeta::File(FileMeta {
                                    name: "movie.mp4".into(),
                                    size: 521425,
                                }),
                            );
                            f.insert(
                                68854343,
                                EntityMeta::File(FileMeta {
                                    name: "music.mp3".into(),
                                    size: 33521,
                                }),
                            );
                            f.insert(
                                157899765,
                                EntityMeta::File(FileMeta {
                                    name: "word.doc".into(),
                                    size: 14512,
                                }),
                            );
                            f.insert(
                                678534342,
                                EntityMeta::File(FileMeta {
                                    name: "image.png".into(),
                                    size: 21512,
                                }),
                            );
                            f.insert(
                                678534342,
                                EntityMeta::Text(TextMeta {
                                    name: "text".into(),
                                }),
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
                    loading: true,
                    query: None,
                }
            };
        }

        &*PLACEHOLDER_GUEST
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClientInfo {
    pub token: AccessTokenEncoded,
    pub id: ClientId,
    pub role: ClientRole,
}
