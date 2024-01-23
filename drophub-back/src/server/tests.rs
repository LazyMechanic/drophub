use assert_matches::assert_matches;
use drophub::{Entity, FileMeta, RoomEvent, RoomOptions, RoomRpcClient};
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
    assert_matches!(sub.next().await, Some(Ok(RoomEvent::Init { .. })));
    assert_matches!(
        sub.next().await,
        Some(Ok(RoomEvent::RoomInfo(info))) if info.clients.len() == 1
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

    let RoomEvent::Init{token: host_token, ..} = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    let RoomEvent::RoomInfo(room_info) = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };

    // Connect without invite
    assert_matches!(client.connect(room_info.id, "123".into()).await, Err(_));

    let invite = client.invite(host_token).await.unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.invites.len() == 1);
    let mut guest1_sub = client
        .connect(invite.room_id, invite.password.clone())
        .await
        .unwrap();
    assert_matches!(guest1_sub.next().await, Some(Ok(RoomEvent::Init { .. })));
    assert_matches!(guest1_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.clients.len() == 2);
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.clients.len() == 2);

    // Connect by used invite
    assert_matches!(client.connect(room_info.id, invite.password).await, Err(_));
}

#[tokio::test]
async fn invite() {
    let cfg = test_utils::test_config();
    let (addr, _h) = server::run(&cfg).await.unwrap();
    let client = WsClientBuilder::default()
        .build(format!("ws://{addr}"))
        .await
        .unwrap();

    let mut host_sub = client
        .create(RoomOptions {
            encryption: false,
            capacity: 2,
        })
        .await
        .unwrap();

    let RoomEvent::Init{token: host_token, ..} = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(_))));

    // Success connect via invite
    let invite = client.invite(host_token.clone()).await.unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.invites.len() == 1);

    let mut guest_sub = client
        .connect(invite.room_id, invite.password)
        .await
        .unwrap();
    let RoomEvent::Init{token: guest_token, ..} = guest_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };

    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.invites.len() == 0 && info.clients.len() == 2);
    assert_matches!(guest_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.invites.len() == 0 && info.clients.len() == 2);

    // Try create invite by guest
    assert_matches!(client.invite(guest_token).await, Err(_));

    // Maximum invites reached
    assert_matches!(client.invite(host_token).await, Err(_));
}

#[tokio::test]
async fn invite_revoke() {
    let cfg = test_utils::test_config();
    let (addr, _h) = server::run(&cfg).await.unwrap();
    let client = WsClientBuilder::default()
        .build(format!("ws://{addr}"))
        .await
        .unwrap();

    let mut host_sub = client
        .create(RoomOptions {
            encryption: false,
            capacity: 3,
        })
        .await
        .unwrap();

    let RoomEvent::Init{token: host_token, ..} = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(_))));

    let invite1 = client.invite(host_token.clone()).await.unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.invites.len() == 1);

    let invite2 = client.invite(host_token.clone()).await.unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.invites.len() == 2);

    assert_matches!(client.invite(host_token.clone()).await, Err(_));

    assert_matches!(
        client
            .revoke_invite(host_token.clone(), invite1.password)
            .await,
        Ok(_)
    );
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.invites.len() == 1);

    assert_matches!(
        client
            .revoke_invite(host_token.clone(), invite2.password)
            .await,
        Ok(_)
    );
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.invites.len() == 0);

    assert_matches!(
        client.revoke_invite(host_token.clone(), "123".into()).await,
        Err(_)
    );
}

#[tokio::test]
async fn kick() {
    let cfg = test_utils::test_config();
    let (addr, _h) = server::run(&cfg).await.unwrap();
    let client = WsClientBuilder::default()
        .build(format!("ws://{addr}"))
        .await
        .unwrap();

    let mut host_sub = client.create(RoomOptions::default()).await.unwrap();

    let RoomEvent::Init{token: host_token, ..} = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(_))));

    let invite = client.invite(host_token.clone()).await.unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.invites.len() == 1);
    let mut guest1_sub = client
        .connect(invite.room_id, invite.password.clone())
        .await
        .unwrap();
    assert_matches!(guest1_sub.next().await, Some(Ok(RoomEvent::Init { .. })));
    assert_matches!(guest1_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.clients.len() == 2);
    let RoomEvent::RoomInfo(room_info) = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };

    assert_matches!(
        client
            .kick(
                host_token.clone(),
                room_info
                    .clients
                    .into_iter()
                    .filter(|c| c != &room_info.host)
                    .next()
                    .unwrap(),
            )
            .await,
        Ok(_)
    );

    assert_matches!(client.kick(host_token, room_info.host).await, Err(_));
}

#[tokio::test]
async fn announce_entity() {
    let cfg = test_utils::test_config();
    let (addr, _h) = server::run(&cfg).await.unwrap();
    let client = WsClientBuilder::default()
        .build(format!("ws://{addr}"))
        .await
        .unwrap();

    let mut host_sub = client.create(RoomOptions::default()).await.unwrap();
    let RoomEvent::Init{token: host_token, ..} = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(_))));

    let file_id = client
        .announce_entity(
            host_token,
            Entity::File(FileMeta {
                name: "123".to_owned(),
                size: 123,
            }),
        )
        .await
        .unwrap();

    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.entities.contains_key(&file_id));
}

#[tokio::test]
async fn remove_entity() {
    let cfg = test_utils::test_config();
    let (addr, _h) = server::run(&cfg).await.unwrap();
    let client = WsClientBuilder::default()
        .build(format!("ws://{addr}"))
        .await
        .unwrap();

    let mut host_sub = client.create(RoomOptions::default()).await.unwrap();
    let RoomEvent::Init{token: host_token, ..} = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(_))));

    let invite = client.invite(host_token.clone()).await.unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.invites.len() == 1);

    let mut guest1_sub = client
        .connect(invite.room_id, invite.password.clone())
        .await
        .unwrap();
    let RoomEvent::Init{token: guest1_token, ..} = guest1_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_matches!(guest1_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.clients.len() == 2);
    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.clients.len() == 2);

    // File doesn't exists
    assert_matches!(client.remove_entity(host_token.clone(), 123).await, Err(_));

    // Announce new file
    let file_id = client
        .announce_entity(
            host_token.clone(),
            Entity::File(FileMeta {
                name: "123".to_owned(),
                size: 123,
            }),
        )
        .await
        .unwrap();

    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.entities.contains_key(&file_id));
    assert_matches!(guest1_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if info.entities.contains_key(&file_id));

    // The owner of the file is another client
    assert_matches!(client.remove_entity(guest1_token, file_id).await, Err(_));
    assert_matches!(client.remove_entity(host_token, file_id).await, Ok(_));

    assert_matches!(host_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if !info.entities.contains_key(&file_id));
    assert_matches!(guest1_sub.next().await, Some(Ok(RoomEvent::RoomInfo(info))) if !info.entities.contains_key(&file_id));
}
