use anyhow::Result;
use smol::process::Command;
use smol::stream::StreamExt;
use smol::unblock;
use smol::Timer;
use smol::{stream, Task};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    // smol::block_on(async {
    //     let out = unblock(|| Command::new("ls").output()).await.await?;
    //     Ok(())
    // })

    smol::block_on(async {
        let mut s = stream::iter(1..=100)
            .map(|x| async move {
                smol::spawn(async move {
                    Timer::after(Duration::from_secs(2));
                    println!("haha{}", x)
                });
                return format!("{}", x);
            })
            .buffer_unordered(5);

        let items: Vec<_> = s.collect().await;
        for i in items {
            println!("{}", i.await)
        }
    });
    // let mut list = vec![];
    // for i in 0..100 {
    //     list.push(i)
    // }
    Ok(())
}
