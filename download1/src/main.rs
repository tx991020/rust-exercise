use anyhow::Error;
use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;
use std::io;
use tokio::fs;
use tokio::io::AsyncWriteExt;

//download
#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let source = client
        .get("http://down.sandai.net/mac/thunder_3.4.1.4368.dmg")
        .send()
        .await?
        .bytes()
        .await?;
    let mut dest = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/Users/andy/Documents/thunder3.zip")
        .await?;

    // while let Some(chunk) = source.chunk().await? {
    //     dest.write_all(&chunk).await?;
    // }
    dest.write_all(source.as_ref()).await?;

    Ok(())
}

// fn upload() -> Result<(), Box<dyn std::error::Error>> {
//     let dest = fs::OpenOptions::new()
//         .open("/Users/andy/CLionProjects/todo/download1/helm.zip")
//         .await?;
//     let mut source = client
//         .post("https://get.helm.sh/helm-v3.2.4-darwin-amd64.tar.gz")
//         .body(dest)
//         .send()
//         .await?;
//     Ok(())
// }
