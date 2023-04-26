use assert_matches::assert_matches;
use drophub::{ClientEvent, RoomInfo, RoomOptions, RoomRpcClient};
use jsonrpsee::ws_client::WsClientBuilder;

use crate::{server, test_utils};

#[tokio::test]
async fn create() {
    let cfg = test_utils::test_config();
    let (addr, _h) = server::run(&cfg).await.unwrap();
    let client = WsClientBuilder::default()
        .build(format!("ws://{addr}"))
        .await
        .unwrap();

    let mut sub = client.create(RoomOptions::default()).await.unwrap();
    assert_matches!(sub.next().await, Some(Ok(ClientEvent::Init(_))));
    assert_matches!(
        sub.next().await,
        Some(Ok(ClientEvent::RoomInfo(info))) if info.clients.len() == 1
    );
}
