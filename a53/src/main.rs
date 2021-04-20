use smol::stream::{self, StreamExt};
use smol::{future, Executor};
fn main() {
    smol::block_on(async {
        let v = stream::iter(1..=3).map(|x| x + 3).collect::<Vec<_>>().await;
        println!("{:?}", v);
    })
}
