use async_channel::{bounded, Receiver, Sender};
use smol::stream;
use smol::stream::StreamExt;
use smol::{Task, Timer};
use std::time::{Duration, Instant};
fn main() {
    smol::block_on(async {
        let (s, r) = bounded(10);

        for i in 0..10 {
            let tx = r.clone();
            Task::spawn(async move {
                loop {
                    let x = tx.clone().recv().await.unwrap();
                    println!("{:?}", Instant::now());
                    Timer::new(Duration::from_secs(1)).await;
                }
            })
            .detach();
        }

        for i in 0..100 {
            s.send(i).await;
        }

        let task = smol::spawn(async { 1 + 2 });

        smol::block_on(async {
            assert_eq!(task.await, 3);
        });
        loop {}
    })
}
