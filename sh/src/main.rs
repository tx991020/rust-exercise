use anyhow::{Error, Result};
use shellfn::shell;

#[shell]
fn list_modified(dir: &str) -> Result<impl Iterator<Item = String>, Error> {
    r#"
    cd $DIR
    git status | grep '^\s*modified:' | awk '{print $2}'
"#
}

fn main() -> Result<()> {
    let result = list_modified("/Users/andy/CLionProjects/todo/sh")?;
    for i in result {
        println!("{}", i);
    }
    Ok(())
}
