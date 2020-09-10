use serde::{Deserialize, Serialize};
use std::io::Read;
use std::str::FromStr;
use tide::prelude::*;
use tide::{sse, Body, Request, Response};

#[derive(Deserialize, Serialize)]
struct Cat {
    name: String,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/sse").get(sse::endpoint(|_req, sender| async move {
        sender.send("fruit", "banana", None).await?;
        sender.send("fruit", "apple", None).await?;
        Ok(())
    }));

    // c.Writer.Header().Add("Content-Disposition", fmt.Sprintf("attachment; filename=%s", filename))//fmt.Sprintf("attachment; filename=%s", filename)对下载的文件重命名
    // c.Writer.Header().Add("Content-Type", "application/octet-stream")
    app.at("/download").get(|_| async {
        let mut res = Response::new(200);
        res.insert_header("Content-Type", "application/zip");
        res.insert_header(
            "Content-Disposition",
            format!("attachment; filename={}", "update.zip"),
        );
        let mut file = std::fs::File::open("/Users/andy/Documents/thunder3.zip").unwrap();
        let mut contents: Vec<u8> = vec![];
        file.read_to_end(&mut contents).unwrap();
        res.set_body(contents);
        Ok(res)
    });
    app.at("/submit").post(|mut req: Request<()>| async move {
        let cat: Cat = req.body_json().await?;
        println!("cat name: {}", cat.name);

        let cat = Cat {
            name: "chashu".into(),
        };

        Ok(Body::from_json(&cat)?)
    });

    app.at("/animals").get(|_| async {
        Ok(json!({
            "meta": { "count": 2 },
            "animals": [
                { "type": "cat", "name": "chashu" },
                { "type": "cat", "name": "nori" }
            ]
        }))
    });

    app.listen("localhost:8080").await?;
    Ok(())
}
