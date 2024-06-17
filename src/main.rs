use colored::Colorize;
use watcher::{cli::cli, parser::parser::parse};

fn main() {
    let result = parse();
    match result {
        Ok(_) => (),
        Err(e) => {
            println!("{}", format!("Error: {}", e).red().bold());
            std::process::exit(1);
        }
    }
    cli::cli();
}
