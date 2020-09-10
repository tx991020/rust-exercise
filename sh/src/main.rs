use shellfn::shell;
use std::error::Error;

#[shell]
fn list_modified(dir: &str) -> Result<impl Iterator<Item = String>, Box<Error>> {
    r#"
    cd $DIR
    git status | grep '^\s*modified:' | awk '{print $2}'
"#
}
