use clap::{self, Arg, ArgMatches, Command};

use crate::watchlib::watch::start_watch;

pub fn cli() {
    let matches = Command::new("Watcher")
        .version("0.0.1")
        .author("urizen")
        .about("A file watcher used to restart applications on changes.")
        .subcommand(
            Command::new("watch")
                .about("Starts the file watcher")
                .arg(Arg::new("directory").index(1).required(true)),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("watch", sub_m)) => call_watch(sub_m),
        None => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}

fn call_watch(args: &ArgMatches) {
    let directory: &str = args.get_one::<String>("directory").unwrap();
    start_watch(directory)
}
