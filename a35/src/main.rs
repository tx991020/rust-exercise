use futures::executor;
use futures::stream::StreamExt;
use std::thread;
use std::time::Duration;

fn main() {
    let mut paths = vec![];
    for i in 0..100 {
        paths.push(format!("{}", i))
    }

    executor::block_on(async {
        let fetches = futures::stream::iter(paths.into_iter().map(|path| async move {
            match reqwest::get(&path).await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => {
                        println!("RESPONSE: {} bytes from {}", text.len(), path);
                    }
                    Err(_) => {
                        println!("ERROR reading {}", path);
                        thread::sleep(Duration::from_secs(1));
                    }
                },
                Err(_) => {
                    println!("ERROR downloading {}", path);
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }))
        .buffer_unordered(3)
        .collect::<Vec<()>>();
        println!("Waiting...");
        fetches.await;
    });
}
