use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    ops::Add,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use anyhow::anyhow;
use drophub::{
    ClientId, DownloadProcId, FileData, FileId, FileMeta, Invite, InviteId, RoomError, RoomId,
    RoomInfo, RoomOptions, UploadRequest,
};
use passwords::PasswordGenerator;
use time::OffsetDateTime;
use tokio::sync::{broadcast, mpsc};
use ttl_cache::TtlCache;
use uuid::Uuid;

use crate::jwt::ClientRole;

pub struct Room {
    pub id: RoomId,
    pub host_id: ClientId,
    pub clients: HashMap<ClientId, Client>,
    pub files: HashMap<FileId, File>,
    pub invites: TtlCache<InviteId, Invite>,
    pub downloads: HashMap<DownloadProcId, DownloadProc>,
    pub encryption: bool,
    pub capacity: usize,
    pub block_size: usize,
    pub info_tx: broadcast::Sender<RoomInfo>,
    pub info_rx: broadcast::Receiver<RoomInfo>,
}

impl Debug for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Room")
            .field("id", &self.id)
            .field("host_id", &self.host_id)
            .field("clients", &self.clients)
            .field("files", &self.files)
            .field("invites", &"{ ... }")
            .field("downloads", &self.downloads)
            .field("encryption", &self.encryption)
            .field("capacity", &self.capacity)
            .field("block_size", &self.block_size)
            .field("info_tx", &"{ ... }")
            .field("info_rx", &"{ ... }")
            .finish()
    }
}

impl Room {
    pub fn new(options: RoomOptions, block_size: usize, host: Client) -> Self {
        let (info_tx, info_rx) = broadcast::channel(5);

        Self {
            id: Self::next_id(),
            host_id: host.id,
            clients: {
                let mut c = HashMap::new();
                c.insert(host.id, host);
                c
            },
            files: Default::default(),
            invites: TtlCache::new(usize::MAX),
            downloads: Default::default(),
            encryption: options.encryption,
            capacity: options.capacity,
            block_size,
            info_tx,
            info_rx,
        }
    }

    fn next_id() -> RoomId {
        static ID: AtomicU64 = AtomicU64::new(0);
        ID.fetch_add(1, Ordering::Relaxed)
    }

    pub fn info(&mut self) -> RoomInfo {
        RoomInfo {
            room_id: self.id,
            host_id: self.host_id,
            files: self
                .files
                .iter()
                .map(|(&file_id, file)| (file_id, file.meta.clone()))
                .collect(),
            clients: self
                .clients
                .iter()
                .map(|(&client_id, _)| client_id)
                .collect(),
            invites: self
                .invites
                .iter()
                .map(|(invite_id, _)| invite_id.clone())
                .collect(),
        }
    }

    pub fn generate_invite(&self, invite_duration: Duration) -> Result<Invite, RoomError> {
        let id_gen = PasswordGenerator {
            length: 8,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: false,
            symbols: false,
            spaces: false,
            exclude_similar_characters: true,
            strict: true,
        };

        let invite_id = id_gen.generate_one().map_err(|err| anyhow!("{}", err))?;

        Ok(Invite {
            id: invite_id,
            room_id: self.id,
            exp: OffsetDateTime::now_utc().add(invite_duration),
        })
    }
}

#[derive(Debug)]
pub struct Client {
    pub id: ClientId,
    pub role: ClientRole,
    pub upload_tx: mpsc::UnboundedSender<UploadRequest>,
}

impl Client {
    pub fn new(role: ClientRole, upload_tx: mpsc::UnboundedSender<UploadRequest>) -> Self {
        Self {
            id: Self::next_id(),
            role,
            upload_tx,
        }
    }

    fn next_id() -> ClientId {
        Uuid::new_v4()
    }
}

#[derive(Debug)]
pub struct File {
    pub id: FileId,
    pub meta: FileMeta,
    pub owner: ClientId,
}

impl File {
    pub fn new(meta: FileMeta, owner: ClientId) -> Self {
        Self {
            id: Self::next_id(),
            meta,
            owner,
        }
    }

    fn next_id() -> FileId {
        Uuid::new_v4()
    }
}

#[derive(Debug)]
pub struct DownloadProc {
    pub id: DownloadProcId,
    pub file_id: FileId,
    pub next_block_idx: usize,
    pub blocks_count: usize,
    pub data_tx: mpsc::Sender<FileData>,
}

impl DownloadProc {
    pub fn new(
        file_id: FileId,
        file_size: usize,
        block_size: usize,
        data_tx: mpsc::Sender<FileData>,
    ) -> Self {
        Self {
            id: Self::next_id(),
            file_id,
            next_block_idx: 0,
            blocks_count: (file_size + block_size - 1) / block_size,
            data_tx,
        }
    }

    fn next_id() -> DownloadProcId {
        Uuid::new_v4()
    }
}
