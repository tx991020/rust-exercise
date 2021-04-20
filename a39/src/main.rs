use signal_hook;

use smol::net::unix::UnixStream;
use smol::{prelude::*, Async};

#[cfg(unix)]
fn main() -> std::io::Result<()> {
    smol::block_on(async {
        // Create a Unix stream that receives a byte on each signal occurrence.
        let (a, mut b) = <UnixStream>::pair()?;
        signal_hook::pipe::register(signal_hook::SIGINT, a)?;
        println!("Waiting for Ctrl-C...");

        // Receive a byte that indicates the Ctrl-C signal occurred.
        b.read_exact(&mut [0]).await?;

        println!("Done!");
        Ok(())
    })
}
