use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter},
    ops::Add,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use anyhow::anyhow;
use drophub::{
    ClientId, DownloadProcId, FileData, FileId, FileMeta, Invite, InvitePassword, RoomError,
    RoomId, RoomInfo, RoomOptions, UploadRequest,
};
use passwords::PasswordGenerator;
use replace_with::replace_with_or_default;
use time::OffsetDateTime;
use tokio::sync::{broadcast, mpsc};
use ttl_cache::TtlCache;
use uuid::Uuid;

use crate::{config::RoomConfig, jwt::ClientRole};

pub struct Room {
    id: RoomId,
    host_id: ClientId,
    clients: HashMap<ClientId, Client>,
    files: HashMap<FileId, File>,
    invites: TtlCache<InvitePassword, Invite>,
    downloads: HashMap<DownloadProcId, DownloadProc>,
    encryption: bool,
    capacity: usize,
    block_size: usize,
    invite_ttl: Duration,
    info_tx: broadcast::Sender<RoomInfo>,
    invite_gen: PasswordGenerator,
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
            .field("invite_ttl", &self.invite_ttl)
            .field("info_tx", &"{ ... }")
            .field("invite_gen", &self.invite_gen)
            .finish()
    }
}

impl Room {
    pub fn new(options: RoomOptions, host: Client, cfg: &RoomConfig) -> Self {
        let (info_tx, _) = broadcast::channel(5);

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

    pub fn host_id(&self) -> ClientId {
        self.host_id
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn broadcast_info(&mut self) -> Result<(), RoomError> {
        let room_info = self.info();
        tracing::info!(?room_info, "Broadcast room info");

        self.info_tx
            .send(room_info)
            .map_err(|_| anyhow!("Room info channel closed"))?;

        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RoomInfo> {
        self.info_tx.subscribe()
    }

    pub fn generate_invite(&mut self) -> Result<Invite, RoomError> {
        loop {
            let invite_password = self
                .invite_gen
                .generate_one()
                .map_err(|err| anyhow!("{}", err))?;

            if !self.invites.contains_key(&invite_password) {
                let invite = Invite {
                    password: invite_password.clone(),
                    room_id: self.id,
                    exp: OffsetDateTime::now_utc().add(self.invite_ttl),
                };
                self.invites
                    .insert(invite_password, invite.clone(), self.invite_ttl);
                break Ok(invite);
            }
        }
    }

    pub fn revoke_invite(&mut self, invite_id: InvitePassword) -> Result<(), RoomError> {
        if self.invites.remove(&invite_id).is_none() {
            return Err(RoomError::InviteNotFound {
                invite_password: invite_id,
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
        let file = self.files.remove(&file_id).ok_or(RoomError::FileNotFound {
            file_id,
            room_id: self.id,
        })?;

        let client = self.get_client_mut(file.owner)?;
        client.files.remove(&file_id);

        Ok(())
    }

    pub fn add_client(
        &mut self,
        client: Client,
        invite_id: InvitePassword,
    ) -> Result<broadcast::Receiver<RoomInfo>, RoomError> {
        let _ = self
            .invites
            .remove(&invite_id)
            .ok_or(RoomError::InviteNotFound {
                invite_password: invite_id,
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
        download_proc_id: DownloadProcId,
    ) -> Result<(), RoomError> {
        let block_size = self.block_size;
        let download_proc = self.get_download_proc_mut(download_proc_id)?;

        if file_data.len() > block_size
            || (file_data.len() < block_size && !download_proc.is_last_block())
        {
            return Err(RoomError::InvalidFileBlockSize {
                file_id: download_proc.file_id,
                recv_block_size: file_data.len(),
                exp_block_size: block_size,
                room_id: self.id,
            });
        }

        let status = download_proc.send_data(file_data).await?;

        match status {
            FileStatus::InProcess { next_req } => {
                let file_id = download_proc.file_id;
                let file = self.get_file(file_id)?;
                let file_owner = self.get_client(file.owner)?;
                file_owner.req_upload(next_req)?;
            }
            FileStatus::FullUploaded => {
                self.downloads.remove(&download_proc_id);
            }
        }

        Ok(())
    }

    pub fn start_download_proc(
        &mut self,
        file_id: FileId,
    ) -> Result<(DownloadProcId, mpsc::Receiver<FileData>), RoomError> {
        let file = self.get_file(file_id)?;
        let file_owner = self.get_client(file.owner)?;

        let (data_tx, data_rx) = mpsc::channel(1);
        let download_proc = DownloadProc::new(file_id, file.meta.size, self.block_size, data_tx);
        tracing::info!(?download_proc, "Init download process");

        let download_proc_id = download_proc.id;
        file_owner.req_upload(UploadRequest {
            download_proc_id,
            file_id,
            block_idx: 0,
        })?;

        self.downloads.insert(download_proc.id, download_proc);

        Ok((download_proc_id, data_rx))
    }

    pub fn stop_download_proc(&mut self, download_proc_id: DownloadProcId) {
        self.downloads.remove(&download_proc_id);
    }

    pub fn is_file_owner(&self, file_id: FileId, client_id: ClientId) -> Result<bool, RoomError> {
        let file = self.get_file(file_id)?;
        Ok(client_id == file.owner)
    }

    pub fn is_file_exists(&self, file_id: FileId) -> bool {
        self.files.contains_key(&file_id)
    }

    pub fn is_full(&mut self) -> bool {
        // TODO: optimize `self.invites.iter().count()`
        self.invites.iter().count() + self.clients.len() >= self.capacity
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

    fn get_file(&self, file_id: FileId) -> Result<&File, RoomError> {
        self.files.get(&file_id).ok_or(RoomError::FileNotFound {
            file_id,
            room_id: self.id,
        })
    }

    fn get_download_proc_mut(
        &mut self,
        download_proc_id: DownloadProcId,
    ) -> Result<&mut DownloadProc, RoomError> {
        self.downloads
            .get_mut(&download_proc_id)
            .ok_or(RoomError::DownloadProcessNotFound {
                download_proc_id,
                room_id: self.id,
            })
    }

    fn get_client(&self, client_id: ClientId) -> Result<&Client, RoomError> {
        self.clients
            .get(&client_id)
            .ok_or(RoomError::ClientNotFound {
                client_id,
                room_id: self.id,
            })
    }

    fn get_client_mut(&mut self, client_id: ClientId) -> Result<&mut Client, RoomError> {
        self.clients
            .get_mut(&client_id)
            .ok_or(RoomError::ClientNotFound {
                client_id,
                room_id: self.id,
            })
    }
}

#[derive(Debug)]
pub struct Client {
    id: ClientId,
    role: ClientRole,
    files: HashSet<FileId>,
    upload_tx: mpsc::UnboundedSender<UploadRequest>,
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

    pub fn id(&self) -> ClientId {
        self.id
    }

    pub fn role(&self) -> ClientRole {
        self.role
    }

    fn req_upload(&self, req: UploadRequest) -> Result<(), RoomError> {
        self.upload_tx
            .send(req)
            .map_err(|_| anyhow!("Upload request channel closed").into())
    }

    fn next_id() -> ClientId {
        Uuid::new_v4()
    }
}

#[derive(Debug)]
pub struct File {
    id: FileId,
    meta: FileMeta,
    owner: ClientId,
}

impl File {
    pub fn new(meta: FileMeta, owner: ClientId) -> Self {
        Self {
            id: meta.checksum,
            meta,
            owner,
        }
    }

    pub fn id(&self) -> FileId {
        self.id
    }
}

#[derive(Debug)]
struct DownloadProc {
    id: DownloadProcId,
    file_id: FileId,
    next_block_idx: usize,
    blocks_count: usize,
    data_tx: mpsc::Sender<FileData>,
}

impl DownloadProc {
    fn new(
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

    fn is_last_block(&self) -> bool {
        self.next_block_idx + 1 == self.blocks_count
    }

    async fn send_data(&mut self, file_data: FileData) -> Result<FileStatus, RoomError> {
        if self.next_block_idx == self.blocks_count {
            return Ok(FileStatus::FullUploaded);
        }

        self.data_tx
            .send(file_data)
            .await
            .map_err(|_| anyhow!("Data channel closed"))?;

        self.next_block_idx += 1;

        if self.next_block_idx == self.blocks_count {
            Ok(FileStatus::FullUploaded)
        } else {
            Ok(FileStatus::InProcess {
                next_req: UploadRequest {
                    download_proc_id: self.id,
                    file_id: self.file_id,
                    block_idx: self.next_block_idx,
                },
            })
        }
    }

    fn next_id() -> DownloadProcId {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum FileStatus {
    InProcess { next_req: UploadRequest },
    FullUploaded,
}
