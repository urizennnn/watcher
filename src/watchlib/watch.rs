use notify::{Config, Event, EventKind, PollWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn start_watch() {
    let (tx, rx) = channel();

    let mut watcher = PollWatcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_secs(3)),
    )
    .expect("Failed to create watcher");

    let home_dir = std::env::var("HOME").expect("Failed to get home directory");
    let watch_dir = format!("{}/morse-translator", home_dir);

    if !Path::new(&watch_dir).exists() {
        eprintln!("Directory doesn't exist");
        return;
    }

    watcher
        .watch(Path::new(&watch_dir), RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    println!("Watching directory...");

    loop {
        match rx.recv() {
            Ok(event) => match event {
                Ok(Event { kind, paths, .. }) => {
                    for path in paths {
                        match kind {
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
                                println!("Other event: {:?}, {:?}", kind, path);
                            }
                        }
                    }
                }
                Err(e) => println!("Event error: {:?}", e),
            },
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
                return;
            }
        }
    }
}
