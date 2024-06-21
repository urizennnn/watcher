use crate::{parser::parser::parse, watchlib::watch::start_watch};
use clap::{self, Arg, ArgAction, Command};
// use once_cell::sync::Lazy;
use std::env;

// static CONFIG: Lazy<Config> = Lazy::new(|| parse().expect("Failed to parse Config file"));

pub fn cli() {
    let matches = Command::new("Watcher")
        .version("0.0.1")
        .author("urizen")
        .about("A file watcher used to restart applications on changes.")
        .arg(
            Arg::new("dev")
                .long("dev")
                .conflicts_with("prod")
                .action(ArgAction::SetTrue),
        )
        .about("Run in development mode according to the config.")
        .arg(
            Arg::new("prod")
                .long("prod")
                .conflicts_with("dev")
                .action(ArgAction::SetTrue),
        )
        .about("Run in production mode according to the config.")
        .arg(Arg::new("directory").index(1).required(false))
        .about("Directory to watch")
        .get_matches();
    let config = parse().unwrap().name;
    if matches.get_flag("dev") {
        println!("{}", config);
        return;
    }

    if let Some(directory) = matches.get_one::<String>("directory") {
        start_watch(directory);
    } else {
        if let Ok(cwd) = env::current_dir() {
            start_watch(cwd.to_str().unwrap())
        } else {
            eprintln!("Failed to get current working directory");
        }
    }
}
