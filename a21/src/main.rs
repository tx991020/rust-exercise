use anyhow::Result;
use smol::channel::Sender;
use smol::io;
use smol::io::AsyncReadExt;

fn main() -> Result<()> {
    smol::block_on(async {
        let input: &[u8] = b"hello";
        let mut reader = io::BufReader::new(input);

        let mut contents = String::new();
        reader.read_to_string(&mut contents).await;
        println!("{}", contents);

        let task = smol::spawn(async { 1 + 2 });
        let a = task.await;
        println!("{}", a);
    });
    Ok(())
}
