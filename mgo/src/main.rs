use serde::{Deserialize, Serialize};
use tokio::stream::StreamExt;
use wither::bson::{doc, oid::ObjectId};
use wither::mongodb::Client;
use wither::{prelude::*, Result};
// Define a model. Simple as deriving a few traits.
#[derive(Debug, Model, Serialize, Deserialize)]
struct White {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "FromVal")]
    pub from_val: i64,
    #[serde(rename = "UpdateTime")]
    pub update_time: i64,
    #[serde(rename = "ToVal")]
    pub to_val: i64,
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Type")]
    pub type_field: i64,
    #[serde(rename = "CreateTime")]
    pub create_time: i64,
    #[serde(rename = "AccountID")]
    pub account_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Connect & sync indexes.
    let db = Client::with_uri_str("mongodb://localhost:27017/")
        .await?
        .database("mydb");

    // Fetch all users.
    let mut cursor = White::find(&db, None, None).await?;
    while let Some(user) = cursor.next().await {
        println!("111");
        println!("{:?}", user);
    }
    Ok(())
}
