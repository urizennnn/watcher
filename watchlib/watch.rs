use colored::Colorize;
use notify::{Config, Event, EventKind, PollWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

fn build_watch_path(directory: &str) -> String {
    format!("{}", directory)
}

fn handle_event(event: notify::Result<Event>) {
    match event {
        Ok(Event { kind, paths, .. }) => {
            for path in paths {
                match kind {
                    EventKind::Modify(notify::event::ModifyKind::Data(_)) => {
                        println!("Data modified: {:?}", path);
                    }
                    EventKind::Create(_) => {
                        println!("File created: {:?}", path);
                    }
                    EventKind::Remove(_) => {
                        println!("File removed: {:?}", path);
                    }
                    EventKind::Access(_) => {
                        println!("File accessed: {:?}", path);
                    }
                    _ => {}
                }
            }
        }
        Err(e) => eprintln!("Event error: {:?}", e),
    }
}

pub fn start_watch(directory: &str) {
    let (tx, rx) = channel();

    let mut watcher = PollWatcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_secs(3)),
    )
    .expect("Failed to create watcher");

    let watch_dir = build_watch_path(directory);
    let watch_path = Path::new(&watch_dir);

    if !watch_path.exists() {
        eprintln!("Directory '{}' doesn't exist", watch_dir);
        return;
    }

    watcher
        .watch(watch_path, RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    println!("{}", format!("Watching directory '{}'", watch_dir).green());
    loop {
        match rx.recv() {
            Ok(event) => handle_event(event),
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
                return;
            }
        }
    }
}
