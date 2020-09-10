use anyhow::Result;
use smol::process::Command;
use smol::unblock;
fn main() -> Result<()> {
    smol::block_on(async {
        let out = unblock(|| Command::new("ls").output()).await.await?;
        Ok(())
    })
}
