use drophub::Error;
use mongodb::bson::doc;
use tracing::instrument;

use crate::server::{models::Invite, storage::DB_NAME};

#[instrument(skip(client))]
pub async fn add_invite(client: &mongodb::Client, invite: Invite) -> Result<(), Error> {
    client
        .database(DB_NAME)
        .collection::<Invite>("invites")
        .insert_one(invite, None)
        .await
        .map_err(|err| Error::MongodbError {
            message: err.to_string(),
            details: Some(serde_json::json! { "Failed to add invite" }),
        })?;

    Ok(())
}

#[instrument(skip(client))]
pub async fn remove_invite(
    client: &mongodb::Client,
    invite_passphrase: &str,
) -> Result<Option<Invite>, Error> {
    client
        .database(DB_NAME)
        .collection::<Invite>("invites")
        .find_one_and_delete(doc! { "passphrase": invite_passphrase }, None)
        .await
        .map_err(|err| Error::MongodbError {
            message: err.to_string(),
            details: Some(serde_json::json! { "Failed to remove invite" }),
        })
}

#[instrument(skip(client))]
pub async fn has_invite(client: &mongodb::Client, invite_passphrase: &str) -> Result<bool, Error> {
    get_invite(client, invite_passphrase)
        .await
        .map(Option::is_some)
}

#[instrument(skip(client))]
pub async fn get_invite(
    client: &mongodb::Client,
    invite_passphrase: &str,
) -> Result<Option<Invite>, Error> {
    client
        .database(DB_NAME)
        .collection::<Invite>("invites")
        .find_one(doc! { "passphrase": invite_passphrase }, None)
        .await
        .map_err(|err| Error::MongodbError {
            message: err.to_string(),
            details: Some(serde_json::json! { "Failed to get invite" }),
        })
}
