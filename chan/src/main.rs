use anyhow::Result;

use std::time::Duration;
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::time::delay_for;
use tokio::{io, task, time};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // let dating = task::spawn(dating());
    // let copying = task::spawn(copying());

    // for i in 0..10 {
    //     task::spawn(delay());
    // }

    let (mut tx, mut rx) = mpsc::channel(10);

    tokio::spawn(async move {
        loop {
            tokio::select! {
                  maybe_v = rx.recv() => {
                      if let Some(v) = maybe_v {
                          println!("got = {}", v);
                      } else {
                          break;
                      }
                  }

            }
        }
    });

    tokio::spawn(async move {
        for i in 0..100 {
            let res = some_computation(i).await;
            tx.send(res).await.unwrap();
        }
    });

    // for i in 0..100 {
    //     tx.send(i.to_string()).await;
    // }
    // while let Some(res) = rx.recv().await {
    //     println!("got = {}", res);
    // }

    let mut list = vec![];
    for i in 0..10 {
        let t = tokio::spawn(async move {
            delay_for(Duration::from_secs(2)).await;
            println!("hahha{}", "3333");
        });
        list.push(t);
    }
    for i in list {
        i.await;
    }

    Ok(())
}

async fn dating() -> Result<(), std::io::Error> {
    let mut interval = time::interval(Duration::from_secs(2));
    loop {
        interval.tick().await;
        println!("{:?}", time::Instant::now());
        Command::new("date").spawn()?.await?;
    }
    Ok(())
}

async fn copying() -> Result<(), std::io::Error> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    io::copy(&mut stdin, &mut stdout).await?;
    Ok(())
}

async fn delay() -> Result<(), std::io::Error> {
    println!("{:?}", time::Instant::now());
    delay_for(Duration::from_secs(2)).await;

    Ok(())
}
// // 解决方案5
// async fn dating() -> Result<(), std::io::Error> {
//     loop {
//         task::spawn_blocking(|| { std::thread::sleep(Duration::from_secs(1)) }).await?;
//         Command::new("date").spawn()?.await?;
//     }
// }

async fn some_computation(input: u32) -> String {
    format!("the result of computation {}", input)
}
