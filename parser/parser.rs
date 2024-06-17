use serde::Deserialize;
use serde_yaml;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    version: i32,
    description: String,
    index: Option<String>,
    watch: WatchConfig,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WatchConfig {
    prod: Option<String>,
    dev: Option<String>,
    default: String,
    exclude: Vec<String>,
    commands: Option<Commands>,
    debounce: Option<u64>,
    log_level: Option<String>,
    environments: Environments,
    aliases: Option<Aliases>,
    notify_on_success: Option<bool>,
    notify_on_failure: Option<bool>,
    ignore_patterns: Option<Vec<String>>,
    color_output: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Commands {
    before: Option<String>,
    after: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Environments {
    prod: EnvironmentConfig,
    dev: EnvironmentConfig,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct EnvironmentConfig {
    command: String,
    args: Vec<String>,
    env: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Aliases {
    build: String,
    test: String,
    lint: String,
}
pub fn parse() -> Result<(), Box<dyn Error>> {
    let mut file_path = None;

    if let Ok(_) = File::open("watch.yml") {
        file_path = Some("watch.yml");
    } else if let Ok(_) = File::open("watch.yaml") {
        file_path = Some("watch.yaml");
    }

    let file_path = match file_path {
        Some(path) => path,
        None => return Err("No valid configuration file found.".into()),
    };

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let _config: Config = serde_yaml::from_str(&contents)?;

    Ok(())
}
