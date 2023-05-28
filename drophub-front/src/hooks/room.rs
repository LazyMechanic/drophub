use std::{collections::HashMap, rc::Rc};

use drophub::{ClientId, FileMeta, InvitePassword, JwtEncoded, RoomInfo, RoomOptions};
use lazy_static::lazy_static;
use uuid::Uuid;
use yew::prelude::*;
use yewdux::prelude::*;

#[hook]
pub fn use_room_store() -> (Rc<RoomStore>, Dispatch<RoomStore>) {
    use_store::<RoomStore>()
}

#[hook]
pub fn use_room_store_value() -> Rc<RoomStore> {
    use_store_value::<RoomStore>()
}

#[derive(Debug, Clone, PartialEq, Store)]
pub struct RoomStore {
    pub room: RoomState,
    pub selected_invite: Option<InvitePassword>,
}

impl Default for RoomStore {
    fn default() -> Self {
        Self {
            room: RoomState::placeholder_host().clone(),
            selected_invite: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RoomState {
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

impl RoomState {
    pub fn placeholder(role: ClientRole) -> &'static Self {
        match role {
            ClientRole::Host => Self::placeholder_host(),
            ClientRole::Guest => Self::placeholder_guest(),
        }
    }

    pub fn placeholder_host() -> &'static Self {
        lazy_static! {
            static ref ROOM_PLACEHOLDER_HOST: RoomState = {
                let client_id = Uuid::new_v4();
                RoomState {
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
            static ref ROOM_PLACEHOLDER_GUEST: RoomState = {
                let client_id = Uuid::new_v4();
                let host_id = Uuid::new_v4();
                RoomState {
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
