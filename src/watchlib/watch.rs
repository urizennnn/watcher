use notify::{Config, Event, EventKind, PollWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub fn start_watch(directory: &str) {
    let home = std::env::var("HOME").unwrap();
    let new_directory = format!("{}/{}", home, directory);
    match create_watcher(&new_directory) {
        Ok(rx) => {
            println!("Watching directory: {}", new_directory);
            event_loop(rx);
        }
        Err(e) => eprintln!("Error initializing watcher: {}", e),
    }
}

fn create_watcher(directory: &str) -> Result<Receiver<notify::Result<Event>>, String> {
    let (tx, rx) = channel();

    let mut watcher = PollWatcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_secs(3)),
    )
    .map_err(|e| format!("Failed to create watcher: {}", e))?;

    if !Path::new(directory).exists() {
        return Err(format!("Directory doesn't exist: {}", directory));
    }

    watcher
        .watch(Path::new(directory), RecursiveMode::Recursive)
        .map_err(|e| format!("Failed to watch directory: {}", e))?;

    Ok(rx)
}

fn event_loop(rx: Receiver<notify::Result<Event>>) {
    loop {
        match rx.recv() {
            Ok(Ok(event)) => process_event(event),
            Ok(Err(e)) => eprintln!("Event error: {:?}", e),
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
                break;
            }
        }
    }
}

/// Handles a single event.
fn process_event(event: Event) {
    for path in event.paths {
        match event.kind {
            EventKind::Modify(notify::event::ModifyKind::Data(_)) => {
                println!("Data modified: {:?}", path);
            }
            EventKind::Modify(notify::event::ModifyKind::Metadata(_)) => {
                println!("Metadata modified: {:?}", path);
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
            _ => {
                println!("Other event: {:?}, {:?}", event.kind, path);
            }
        }
    }
}
