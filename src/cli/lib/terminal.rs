use std::process::Command;

pub fn run_cli() {
    let output = Command::new("npm")
        .arg("help")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        println!("npm help: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("npm error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
