use colored::Colorize;

use watcher::{cli::cli, parser::parser::parse};

fn main() {
    let result = parse();
    match result {
        Ok(_) => {
            cli::cli();
        }
        Err(e) => {
            let err_message = format!("{}", e);
            if let Some(line) = err_message.find(", expected struct Config") {
                let end = line + ", expected struct Config".len();
                let new_string = format!("{}{}", &err_message[..line], &err_message[end..]);
                println!("{}", new_string.red());
                std::process::exit(1);
            }
        }
    }
    cli::cli();
}
