use drophub::{ClientId, InvitePassword, JwtEncoded, RoomId, RoomInfo, RoomOptions, UploadRequest};
use futures::StreamExt;
use yew::platform::pinned::{mpsc, oneshot};

use crate::error::Error;

#[derive(Debug)]
pub enum RpcRequestMsg {
    Single(RpcSingleRequest, RpcSingleResponseTx),
    Subscribe(RpcSubscribeRequest, RpcSubscribeResponseTx),
}

#[derive(Debug, Clone)]
pub enum RpcResponseMsg {
    Single(RpcSingleResponse),
    Subscribe(RpcSubscribeResponse),
}

#[derive(Debug, Clone)]
pub struct RpcRequestTx(mpsc::UnboundedSender<RpcRequestMsg>);
#[derive(Debug)]
pub struct RpcRequestRx(mpsc::UnboundedReceiver<RpcRequestMsg>);

pub fn channel() -> (RpcRequestTx, RpcRequestRx) {
    let (tx, rx) = mpsc::unbounded();
    (RpcRequestTx(tx), RpcRequestRx(rx))
}

impl RpcRequestTx {
    pub async fn send(&self, req: RpcSingleRequest) -> Result<RpcSingleResponse, Error> {
        let (tx, rx) = resp_channel();
        self.0
            .send_now(RpcRequestMsg::Single(req, tx))
            .map_err(|_| Error::ChannelClosed {
                details: "Sending request via RPC channel".into(),
            })?;

        rx.recv().await
    }

    pub fn sub(&self, req: RpcSubscribeRequest) -> Result<RpcSubscribeResponseRx, Error> {
        let (tx, rx) = sub_channel();
        self.0
            .send_now(RpcRequestMsg::Subscribe(req, tx))
            .map_err(|_| Error::ChannelClosed {
                details: "Sending request via RPC channel".into(),
            })?;

        Ok(rx)
    }
}

impl RpcRequestRx {
    pub async fn recv(&mut self) -> Option<RpcRequestMsg> {
        self.0.next().await
    }
}

// =================================================================================================
// Single
// =================================================================================================

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RpcSingleRequest {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RpcSingleResponse {}

#[derive(Debug)]
pub struct RpcSingleResponseTx(oneshot::Sender<RpcSingleResponse>);
#[derive(Debug)]
pub struct RpcSingleResponseRx(oneshot::Receiver<RpcSingleResponse>);

fn resp_channel() -> (RpcSingleResponseTx, RpcSingleResponseRx) {
    let (tx, rx) = oneshot::channel();
    (RpcSingleResponseTx(tx), RpcSingleResponseRx(rx))
}

impl RpcSingleResponseTx {
    pub fn send(self, resp: RpcSingleResponse) -> Result<(), Error> {
        self.0.send(resp).map_err(|_| Error::ChannelClosed {
            details: "Sending response via RPC channel".into(),
        })
    }
}

impl RpcSingleResponseRx {
    pub async fn recv(self) -> Result<RpcSingleResponse, Error> {
        self.0.await.map_err(|_| Error::ChannelClosed {
            details: "Receiving response via RPC channel".into(),
        })
    }
}

// =================================================================================================
// Subscribe
// =================================================================================================

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RpcSubscribeRequest {
    CreateRoom(RoomOptions),
    ConnectRoom(RoomCredentials),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoomCredentials {
    pub id: RoomId,
    pub password: InvitePassword,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RpcSubscribeResponse {
    Room(RoomMsg),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoomMsg {
    Init(JwtEncoded, ClientId),
    RoomInfo(RoomInfo),
    UploadRequest(UploadRequest),
}

#[derive(Debug, Clone)]
pub struct RpcSubscribeResponseTx(mpsc::UnboundedSender<RpcSubscribeResponse>);
pub struct RpcSubscribeResponseRx(mpsc::UnboundedReceiver<RpcSubscribeResponse>);

fn sub_channel() -> (RpcSubscribeResponseTx, RpcSubscribeResponseRx) {
    let (tx, rx) = mpsc::unbounded();
    (RpcSubscribeResponseTx(tx), RpcSubscribeResponseRx(rx))
}

impl RpcSubscribeResponseTx {
    pub fn send(&self, resp: RpcSubscribeResponse) -> Result<(), Error> {
        self.0.send_now(resp).map_err(|_| Error::ChannelClosed {
            details: "Sending response via RPC subscribe channel".into(),
        })
    }
}

impl RpcSubscribeResponseRx {
    pub async fn recv(&mut self) -> Option<RpcSubscribeResponse> {
        self.0.next().await
    }

    pub async fn try_recv(&mut self) -> Result<RpcSubscribeResponse, Error> {
        self.recv().await.ok_or_else(|| Error::ChannelClosed {
            details: "Receiving subscription response".into(),
        })
    }
}
