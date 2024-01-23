use std::collections::HashSet;

use chrono::{DateTime, Duration, Utc};
use drophub::{EntityId, EntityKind, InvitePassphrase, PeerId, RoomId};

pub const INVITE_TTL: Duration = Duration::hours(1);
pub const ROOM_TTL: Duration = Duration::hours(24);
pub const PEER_TTL: Duration = Duration::hours(24);
pub const ENTITY_TTL: Duration = Duration::hours(24);

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Peer {
    pub id: PeerId,
    pub create_at: DateTime<Utc>,
    pub state: PeerState,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum PeerState {
    Disconnected,
    Connecting {
        connecting_at: DateTime<Utc>,
        room_id: RoomId,
    },
    Connected {
        connected_at: DateTime<Utc>,
        room_id: RoomId,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Room {
    pub id: RoomId,
    pub create_at: DateTime<Utc>,
    pub peers: HashSet<PeerId>,
    pub entities: HashSet<EntityId>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub create_at: DateTime<Utc>,
    pub kind: EntityKind,
    pub name: String,
    pub size: usize,
    pub owner_id: PeerId,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Invite {
    pub passphrase: InvitePassphrase,
    pub create_at: DateTime<Utc>,
    pub peer_id: PeerId,
}

impl Invite {
    pub fn is_expired(&self) -> bool {
        self.create_at + INVITE_TTL < Utc::now()
    }
}
