use std::process::{Command, Output};

pub fn restart_app(command: &str) {
    let cmd = Command::new(command)
        .output()
        .expect("Failed to restart the application");
    print!("{:?}", cmd.stdout);
}
