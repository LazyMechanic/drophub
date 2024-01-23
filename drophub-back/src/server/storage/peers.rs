use drophub::{Error, PeerId};
use mongodb::bson::doc;
use tracing::instrument;

use crate::server::{models::Peer, storage::DB_NAME};

#[instrument(skip(client))]
pub async fn remove_peer(client: &mongodb::Client, peer_id: PeerId) -> Result<Option<Peer>, Error> {
    client
        .database(DB_NAME)
        .collection::<Peer>("peers")
        .find_one_and_delete(doc! { "id": peer_id }, None)
        .await
        .map_err(|err| Error::MongodbError {
            message: err.to_string(),
            details: Some(serde_json::json! { "Failed to remove peer" }),
        })
}
