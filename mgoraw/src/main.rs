use mongodb::Cursor;
use mongodb::{bson, bson::Document, Client};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::stream::StreamExt;
#[derive(Deserialize, Serialize, Debug)]
struct White {
    /// The ID of the model.
    #[serde(rename = "_id")]
    pub id: String,
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

//函数式编程,ch,mysql,mgo,click,mio,pg

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
    let db = client.database("mydb");

    let coll = db.collection("whites2");
    let cursor = coll.find(None, None).await?;
    let res: Vec<White> = cursor
        .map(|item| {
            let doc: Document = item.unwrap();
            let bson = bson::Bson::Document(doc);
            return bson::from_bson(bson).unwrap();
        })
        .collect()
        .await;

    println!("{:?}", res[0]);
    /// let results: Vec<Result<Document>> = cursor.collect().await;
    ///
    Ok(())
}
