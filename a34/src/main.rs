use clap::{load_yaml, App};

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    // Same as previous examples...
    match matches.subcommand_name() {
        Some("test") => println!("'myapp add' was used"),
        Some("clone") => {
            println!("{}", "clone");
        } // clone was used
        Some("push") => {
            println!("{}", "push");
        } // push was used
        Some("commit") => {
            println!("{}", "commit");
        } // commit was used

        _ => {}
    }
}
