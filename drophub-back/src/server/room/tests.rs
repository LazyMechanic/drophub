use assert_matches::assert_matches;
use drophub::{ClientEvent, RoomOptions, RoomRpcClient};
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

#[tokio::test]
async fn connect() {
    let cfg = test_utils::test_config();
    let (addr, _h) = server::run(&cfg).await.unwrap();
    let client = WsClientBuilder::default()
        .build(format!("ws://{addr}"))
        .await
        .unwrap();

    let mut host_sub = client.create(RoomOptions::default()).await.unwrap();

    let ClientEvent::Init(host_jwt) = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    let ClientEvent::RoomInfo(room_info) = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };

    // Connect without invite
    assert_matches!(
        client.connect(room_info.room_id, "123".into()).await,
        Err(_)
    );

    let invite = client.invite(host_jwt).await.unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.invites.len() == 1);
    let mut guest1_sub = client
        .connect(invite.room_id, invite.id.clone())
        .await
        .unwrap();
    assert_matches!(guest1_sub.next().await, Some(Ok(ClientEvent::Init(_))));
    assert_matches!(guest1_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.clients.len() == 2);
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.clients.len() == 2);

    // Connect by used invite
    assert_matches!(client.connect(room_info.room_id, invite.id).await, Err(_));
}
