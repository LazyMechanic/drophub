use anyhow::anyhow;
use drophub::RoomOptions;
use futures::StreamExt;
use yew::platform::pinned::{mpsc, oneshot};

type RpcRequestTx = mpsc::UnboundedSender<(RpcRequest, RpcResponseTx)>;
type RpcRequestRx = mpsc::UnboundedReceiver<(RpcRequest, RpcResponseTx)>;

type RpcResponseTx = oneshot::Sender<RpcResponse>;
type RpcResponseRx = oneshot::Receiver<RpcResponse>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RpcRequest {
    Room(RoomRpcModule),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RoomRpcModule {
    Create(RoomOptions),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RpcResponse {}

pub struct RpcRequestSender(RpcRequestTx);
pub struct RpcRequestReceiver(RpcRequestRx);

pub fn channel() -> (RpcRequestSender, RpcRequestReceiver) {
    let (tx, rx) = mpsc::unbounded();
    (RpcRequestSender(tx), RpcRequestReceiver(rx))
}

impl RpcRequestSender {
    pub fn send(&self, req: RpcRequest) -> anyhow::Result<RpcResponseRx> {
        let (resp_tx, resp_rx) = oneshot::channel();
        self.0
            .send_now((req, resp_tx))
            .map_err(|_| anyhow!("Rpc channel closed"))?;

        Ok(resp_rx)
    }
}

impl RpcRequestReceiver {
    pub async fn recv(&mut self) -> Option<(RpcRequest, RpcResponseTx)> {
        self.0.next().await
    }
}
