use clap::{arg, Command, ArgMatches};
use notify::{Error, Watcher};
use std::path::Path;

pub fn make_subcommand() -> Command {
    Command::new("watch")
        .about("Watches blog changes")
        .arg(arg!([dir] "Root directory for the book{n}").default_value("blogs"))
}

pub fn execute(args: &ArgMatches) -> Result<(), Error> {
    let mut watcher = notify::recommended_watcher(|res| match res {
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    })?;
    watcher.watch(Path::new("./blogs"), notify::RecursiveMode::Recursive)?;
    Ok(())
}
