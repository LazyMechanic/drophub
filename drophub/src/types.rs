use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
#[cfg(feature = "rpc-server")]
use jsonrpsee::SubscriptionMessage;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Error;

pub type PeerId = Uuid;
pub type RoomId = Uuid;
pub type EntityId = Uuid;
pub type InvitePassphrase = String;
pub type PeerTokenEncoded = String;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Room {
    pub id: RoomId,
    pub entities: HashMap<EntityId, Entity>,
    pub peers: HashMap<PeerId, Peer>,
}

#[cfg(feature = "rpc-server")]
impl TryFrom<Room> for SubscriptionMessage {
    type Error = serde_json::Error;

    fn try_from(f: Room) -> Result<Self, Self::Error> {
        SubscriptionMessage::from_json(&f)
    }
}

#[cfg(feature = "rpc-server")]
impl TryFrom<&Room> for SubscriptionMessage {
    type Error = <SubscriptionMessage as TryFrom<Room>>::Error;

    fn try_from(f: &Room) -> Result<Self, Self::Error> {
        SubscriptionMessage::from_json(f)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityKind {
    File,
    Text,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub struct Entity {
    pub kind: EntityKind,
    pub name: String,
    pub size: usize,
    pub owner_id: PeerId,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AnnouncedEntity {
    pub kind: EntityKind,
    pub name: String,
    pub size: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Peer {
    pub connected_ts: DateTime<Utc>,
    pub entities: HashSet<EntityId>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum PeerEvent {
    Init {
        token: PeerTokenEncoded,
        invite_passphrase: InvitePassphrase,
    },
    Invite {
        token: PeerTokenEncoded,
    },
    UpdateRoom {
        room: Room,
    },
}

#[cfg(feature = "rpc-server")]
impl TryFrom<PeerEvent> for SubscriptionMessage {
    type Error = serde_json::Error;

    fn try_from(f: PeerEvent) -> Result<Self, Self::Error> {
        SubscriptionMessage::from_json(&f)
    }
}

#[cfg(feature = "rpc-server")]
impl TryFrom<&PeerEvent> for SubscriptionMessage {
    type Error = <SubscriptionMessage as TryFrom<PeerEvent>>::Error;

    fn try_from(f: &PeerEvent) -> Result<Self, Self::Error> {
        SubscriptionMessage::from_json(f)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PeerToken {
    pub peer_id: PeerId,
    pub room_id: Option<RoomId>,
    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub exp: Option<DateTime<Utc>>,
}

impl PeerToken {
    /// Encodes token to JWT format.
    pub fn encode(&self, secret: &str) -> Result<String, Error> {
        let tok = jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        tracing::debug!("Encoded peer token: {:?}", tok);
        Ok(tok)
    }

    /// Decodes token from JWT format and verify signature by secret key.
    pub fn decode_and_verify(token: &str, secret: &str) -> Result<Self, Error> {
        let validation = {
            let mut v = Validation::default();
            v.set_required_spec_claims::<&str>(&[]);
            v
        };

        let tok = jsonwebtoken::decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )
        .map(|token_data| token_data.claims)?;

        tracing::debug!("Decoded peer token: {:?}", tok);
        Ok(tok)
    }

    /// Decodes token from JWT format without verifying.
    pub fn decode(token: &str) -> Result<Self, Error> {
        let validation = {
            let mut v = Validation::default();
            v.set_required_spec_claims::<&str>(&[]);
            v.insecure_disable_signature_validation();
            v
        };

        let tok = jsonwebtoken::decode::<Self>(token, &DecodingKey::from_secret(&[]), &validation)
            .map(|token_data| token_data.claims)?;

        tracing::debug!("Decoded peer token: {:?}", tok);
        Ok(tok)
    }
}
