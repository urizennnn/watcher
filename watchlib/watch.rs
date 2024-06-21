use colored::Colorize;
use notify::{Config, Event, EventKind, PollWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn build_watch_path(directory: &str) -> String {
    directory.to_string()
}

fn handle_event(event: notify::Result<Event>, exclude_dir: &[PathBuf]) {
    match event {
        Ok(Event { kind, paths, .. }) => {
            for path in paths {
                if exclude_dir
                    .iter()
                    .any(|exclude_path| path.starts_with(exclude_path))
                {
                    println!(
                        "{}",
                        "Excluded directory got modified, not restarting".red(),
                    );
                } else {
                    match kind {
                        EventKind::Modify(notify::event::ModifyKind::Data(_)) => {
                            println!("Data modified: {:?}", path);
                        }
                        EventKind::Create(notify::event::CreateKind::File) => {
                            println!("File created: {:?}", path);
                        }
                        EventKind::Remove(_) => {
                            println!("File removed: {:?}", path);
                        }
                        _ => {}
                    }
                }
            }
        }
        Err(e) => eprintln!("Event error: {:?}", e),
    }
}

pub fn start_watch(directory: &str, mut exclude_dir: Vec<PathBuf>) {
    let (tx, rx) = channel();

    let mut watcher = PollWatcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_millis(1000)),
    )
    .expect("Failed to create watcher");

    let watch_path: PathBuf = Path::new(directory).into();
    if !watch_path.exists() {
        eprintln!("Directory '{}' doesn't exist", watch_path.display());
        return;
    }

    exclude_dir = exclude_dir
        .into_iter()
        .map(|ex| watch_path.join(ex))
        .collect();

    watcher
        .watch(&watch_path, RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    println!(
        "{}",
        format!(
            "Watching directory '{}'",
            watch_path.display().to_string().green()
        )
    );
    loop {
        match rx.recv() {
            Ok(event) => handle_event(event, &exclude_dir),
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
                break;
            }
        }
    }
}
