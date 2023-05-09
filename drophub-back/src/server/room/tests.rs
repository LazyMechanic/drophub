use assert_matches::assert_matches;
use drophub::{ClientEvent, FileData, FileMeta, RoomOptions, RoomRpcClient, UploadRequest};
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
        .connect(invite.room_id, invite.password.clone())
        .await
        .unwrap();
    assert_matches!(guest1_sub.next().await, Some(Ok(ClientEvent::Init(_))));
    assert_matches!(guest1_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.clients.len() == 2);
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.clients.len() == 2);

    // Connect by used invite
    assert_matches!(
        client.connect(room_info.room_id, invite.password).await,
        Err(_)
    );
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

    let ClientEvent::Init(host_jwt) = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(_))));

    // Success connect via invite
    let invite = client.invite(host_jwt.clone()).await.unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.invites.len() == 1);

    let mut guest_sub = client
        .connect(invite.room_id, invite.password)
        .await
        .unwrap();
    let ClientEvent::Init(guest_jwt) = guest_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };

    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.invites.len() == 0 && info.clients.len() == 2);
    assert_matches!(guest_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.invites.len() == 0 && info.clients.len() == 2);

    // Try create invite by guest
    assert_matches!(client.invite(guest_jwt).await, Err(_));

    // Maximum invites reached
    assert_matches!(client.invite(host_jwt).await, Err(_));
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

    let ClientEvent::Init(host_jwt) = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(_))));

    let invite1 = client.invite(host_jwt.clone()).await.unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.invites.len() == 1);

    let invite2 = client.invite(host_jwt.clone()).await.unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.invites.len() == 2);

    assert_matches!(client.invite(host_jwt.clone()).await, Err(_));

    assert_matches!(
        client
            .revoke_invite(host_jwt.clone(), invite1.password)
            .await,
        Ok(_)
    );
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.invites.len() == 1);

    assert_matches!(
        client
            .revoke_invite(host_jwt.clone(), invite2.password)
            .await,
        Ok(_)
    );
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.invites.len() == 0);

    assert_matches!(
        client.revoke_invite(host_jwt.clone(), "123".into()).await,
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

    let ClientEvent::Init(host_jwt) = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(_))));

    let invite = client.invite(host_jwt.clone()).await.unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.invites.len() == 1);
    let mut guest1_sub = client
        .connect(invite.room_id, invite.password.clone())
        .await
        .unwrap();
    assert_matches!(guest1_sub.next().await, Some(Ok(ClientEvent::Init(_))));
    assert_matches!(guest1_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.clients.len() == 2);
    let ClientEvent::RoomInfo(room_info) = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };

    assert_matches!(
        client
            .kick(
                host_jwt.clone(),
                room_info
                    .clients
                    .into_iter()
                    .filter(|c| c != &room_info.host_id)
                    .next()
                    .unwrap(),
            )
            .await,
        Ok(_)
    );

    assert_matches!(client.kick(host_jwt, room_info.host_id).await, Err(_));
}

#[tokio::test]
async fn announce_file() {
    let cfg = test_utils::test_config();
    let (addr, _h) = server::run(&cfg).await.unwrap();
    let client = WsClientBuilder::default()
        .build(format!("ws://{addr}"))
        .await
        .unwrap();

    let mut host_sub = client.create(RoomOptions::default()).await.unwrap();
    let ClientEvent::Init(host_jwt) = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(_))));

    let file_id = client
        .announce_file(
            host_jwt,
            FileMeta {
                name: "123".to_owned(),
                size: 123,
                checksum: 123,
            },
        )
        .await
        .unwrap();

    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.files.contains_key(&file_id));
}

#[tokio::test]
async fn remove_file() {
    let cfg = test_utils::test_config();
    let (addr, _h) = server::run(&cfg).await.unwrap();
    let client = WsClientBuilder::default()
        .build(format!("ws://{addr}"))
        .await
        .unwrap();

    let mut host_sub = client.create(RoomOptions::default()).await.unwrap();
    let ClientEvent::Init(host_jwt) = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(_))));

    let invite = client.invite(host_jwt.clone()).await.unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.invites.len() == 1);

    let mut guest1_sub = client
        .connect(invite.room_id, invite.password.clone())
        .await
        .unwrap();
    let ClientEvent::Init(guest1_jwt) = guest1_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_matches!(guest1_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.clients.len() == 2);
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.clients.len() == 2);

    // File doesn't exists
    assert_matches!(client.remove_file(host_jwt.clone(), 123).await, Err(_));

    // Announce new file
    let file_id = client
        .announce_file(
            host_jwt.clone(),
            FileMeta {
                name: "123".to_owned(),
                size: 123,
                checksum: 123,
            },
        )
        .await
        .unwrap();

    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.files.contains_key(&file_id));
    assert_matches!(guest1_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.files.contains_key(&file_id));

    // The owner of the file is another client
    assert_matches!(client.remove_file(guest1_jwt, file_id).await, Err(_));
    assert_matches!(client.remove_file(host_jwt, file_id).await, Ok(_));

    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if !info.files.contains_key(&file_id));
    assert_matches!(guest1_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if !info.files.contains_key(&file_id));
}

#[tokio::test]
async fn download_file() {
    let cfg = test_utils::test_config();
    let (addr, _h) = server::run(&cfg).await.unwrap();
    let client = WsClientBuilder::default()
        .build(format!("ws://{addr}"))
        .await
        .unwrap();

    let mut host_sub = client.create(RoomOptions::default()).await.unwrap();
    let ClientEvent::Init(host_jwt) = host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(_))));

    // Success connect via invite
    let invite = client.invite(host_jwt.clone()).await.unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.invites.len() == 1);

    let mut guest_sub = client
        .connect(invite.room_id, invite.password)
        .await
        .unwrap();
    let ClientEvent::Init(guest_jwt) = guest_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };

    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.invites.len() == 0 && info.clients.len() == 2);
    assert_matches!(guest_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.invites.len() == 0 && info.clients.len() == 2);

    let file_id = client
        .announce_file(
            host_jwt.clone(),
            FileMeta {
                name: "123".to_owned(),
                size: 10000,
                checksum: 123,
            },
        )
        .await
        .unwrap();
    assert_matches!(host_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.files.contains_key(&file_id));
    assert_matches!(guest_sub.next().await, Some(Ok(ClientEvent::RoomInfo(info))) if info.files.contains_key(&file_id));

    let mut download_sub = client.sub_download(guest_jwt, file_id).await.unwrap();
    let ClientEvent::UploadRequest(UploadRequest{ download_proc_id, file_id: upload_file_id, block_idx }) =  host_sub.next().await.unwrap().unwrap() else { panic!("unexpected event") };
    assert_eq!(upload_file_id, file_id);
    assert_eq!(block_idx, 0);

    let full_file = vec![42u8; 10000];
    let block1 = full_file[..cfg.room.block_size].to_vec();
    let block2 = full_file[cfg.room.block_size..].to_vec();

    assert_matches!(
        client
            .upload_file(host_jwt.clone(), FileData(block1.clone()), download_proc_id)
            .await,
        Ok(_)
    );
    assert_matches!(download_sub.next().await, Some(Ok(file_data)) if *file_data == block1);

    assert_matches!(
        client
            .upload_file(host_jwt.clone(), FileData(block2.clone()), download_proc_id)
            .await,
        Ok(_)
    );
    assert_matches!(download_sub.next().await, Some(Ok(file_data)) if *file_data == block2);

    // Download process completed
    assert_matches!(
        client
            .upload_file(host_jwt.clone(), FileData(block2.clone()), download_proc_id)
            .await,
        Err(_)
    );
}
