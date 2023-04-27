use std::{
    collections::{HashMap, HashSet},
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
use replace_with::replace_with_or_default;
use time::OffsetDateTime;
use tokio::sync::{broadcast, mpsc};
use ttl_cache::TtlCache;
use uuid::Uuid;

use crate::{config::RoomConfig, jwt::ClientRole};

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
    pub invite_ttl: Duration,
    pub info_tx: broadcast::Sender<RoomInfo>,
    pub info_rx: broadcast::Receiver<RoomInfo>,
    pub invite_gen: PasswordGenerator,
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
            .field("invite_gen", &self.invite_gen)
            .finish()
    }
}

impl Room {
    pub fn new(options: RoomOptions, host: Client, cfg: &RoomConfig) -> Self {
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
            block_size: cfg.block_size,
            invite_ttl: cfg.invite_ttl,
            info_tx,
            info_rx,
            invite_gen: PasswordGenerator {
                length: 8,
                numbers: true,
                lowercase_letters: true,
                uppercase_letters: false,
                symbols: false,
                spaces: false,
                exclude_similar_characters: true,
                strict: true,
            },
        }
    }

    pub fn id(&self) -> RoomId {
        self.id
    }

    pub fn broadcast_info(&mut self) -> Result<(), RoomError> {
        let room_info = self.info();
        self.info_tx
            .send(room_info)
            .map_err(|_| anyhow!("Room info channel closed"))?;

        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RoomInfo> {
        self.info_tx.subscribe()
    }

    pub fn generate_invite(&mut self) -> Result<Invite, RoomError> {
        // TODO: generate UNIQUE id
        let invite_id = self
            .invite_gen
            .generate_one()
            .map_err(|err| anyhow!("{}", err))?;
        let invite = Invite {
            id: invite_id.clone(),
            room_id: self.id,
            exp: OffsetDateTime::now_utc().add(self.invite_ttl),
        };

        self.invites
            .insert(invite_id, invite.clone(), self.invite_ttl);
        Ok(invite)
    }

    pub fn revoke_invite(&mut self, invite_id: InviteId) -> Result<(), RoomError> {
        if self.invites.remove(&invite_id).is_none() {
            return Err(RoomError::InviteNotFound {
                invite_id,
                room_id: self.id,
            });
        }

        Ok(())
    }

    pub fn add_file(&mut self, file: File) -> Result<(), RoomError> {
        let client = self.get_client_mut(file.owner)?;

        client.files.insert(file.id);
        self.files.insert(file.id, file);

        Ok(())
    }

    pub fn remove_file(&mut self, file_id: FileId) -> Result<(), RoomError> {
        let file = self
            .files
            .remove(&file_id)
            .ok_or_else(|| RoomError::FileNotFound {
                file_id,
                room_id: self.id,
            })?;

        let client = self.get_client_mut(file.owner)?;
        client.files.remove(&file.id);

        Ok(())
    }

    pub fn add_client(
        &mut self,
        client: Client,
        invite_id: InviteId,
    ) -> Result<broadcast::Receiver<RoomInfo>, RoomError> {
        let _ = self
            .invites
            .remove(&invite_id)
            .ok_or(RoomError::InviteNotFound {
                invite_id,
                room_id: self.id,
            })?;

        // Add connected client
        self.clients.insert(client.id, client);
        Ok(self.info_tx.subscribe())
    }

    pub fn remove_client(&mut self, client_id: ClientId) -> Result<(), RoomError> {
        // Remove client from list
        self.clients.remove(&client_id);

        // Remove all client-owned files
        // TODO: don't delete the file if any other client has it
        replace_with_or_default(&mut self.files, |room_files| {
            room_files
                .into_iter()
                .filter(|(_, file)| file.owner != client_id)
                .collect()
        });
        Ok(())
    }

    pub async fn send_file(
        &mut self,
        file_data: FileData,
        download_id: DownloadProcId,
    ) -> Result<(), RoomError> {
        let mut download_proc =
            self.downloads
                .get_mut(&download_id)
                .ok_or(RoomError::DownloadProcessNotFound {
                    download_id,
                    room_id: self.id,
                })?;

        download_proc
            .data_tx
            .send(file_data)
            .await
            .map_err(|_| anyhow!("Data channel closed"))?;

        download_proc.next_block_idx += 1;
        if download_proc.next_block_idx >= download_proc.blocks_count {
            self.downloads.remove(&download_id);
        }

        Ok(())
    }

    pub fn start_download_proc(
        &mut self,
        file_id: FileId,
    ) -> Result<(DownloadProcId, mpsc::Receiver<FileData>), RoomError> {
        let file = self.files.get(&file_id).ok_or(RoomError::FileNotFound {
            file_id,
            room_id: self.id,
        })?;

        let (data_tx, data_rx) = mpsc::channel(1);
        let download_proc = DownloadProc::new(file_id, file.meta.size, self.block_size, data_tx);

        let download_proc_id = download_proc.id;
        self.downloads.insert(download_proc.id, download_proc);

        Ok((download_proc_id, data_rx))
    }

    pub fn stop_download_proc(&mut self, download_proc_id: DownloadProcId) {
        self.downloads.remove(&download_proc_id);
    }

    fn next_id() -> RoomId {
        static ID: AtomicU64 = AtomicU64::new(0);
        ID.fetch_add(1, Ordering::Relaxed)
    }

    fn info(&mut self) -> RoomInfo {
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

    fn get_client_mut(&mut self, client_id: ClientId) -> Result<&mut Client, RoomError> {
        self.clients
            .get_mut(&client_id)
            .ok_or_else(|| RoomError::ClientNotFound {
                client_id,
                room_id: self.id,
            })
    }
}

#[derive(Debug)]
pub struct Client {
    pub id: ClientId,
    pub role: ClientRole,
    pub files: HashSet<FileId>,
    pub upload_tx: mpsc::UnboundedSender<UploadRequest>,
}

impl Client {
    pub fn new(role: ClientRole, upload_tx: mpsc::UnboundedSender<UploadRequest>) -> Self {
        Self {
            id: Self::next_id(),
            role,
            files: Default::default(),
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
            id: meta.checksum,
            meta,
            owner,
        }
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
