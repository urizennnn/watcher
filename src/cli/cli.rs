use crate::watchlib::watch::start_watch;
use clap::{self, Arg, Command};
use std::env;

pub fn cli() {
    let matches = Command::new("Watcher")
        .version("0.0.1")
        .author("urizen")
        .about("A file watcher used to restart applications on changes.")
        .arg(Arg::new("dev").long("dev").conflicts_with("prod"))
        .about("Run in development mode according to the config.")
        .arg(Arg::new("prod").long("prod").conflicts_with("dev"))
        .about("Run in production mode according to the config.")
        .arg(Arg::new("directory").index(1).required(false))
        .get_matches();

    if let Some(directory) = matches.get_one::<String>("directory") {
        start_watch(directory);
    } else {
        if let Ok(cwd) = env::current_dir() {
            start_watch(cwd.to_str().unwrap());
        } else {
            eprintln!("Failed to get current working directory");
        }
    }
}
