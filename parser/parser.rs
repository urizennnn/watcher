use serde::Deserialize;
use serde_yaml;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: i32,
    pub description: String,
    pub index: Option<String>,
    pub watch: WatchConfig,
}

#[derive(Debug, Deserialize)]
pub struct WatchConfig {
    pub prod: Option<String>,
    pub dev: Option<String>,
    pub default: String,
    pub exclude: Vec<String>,
    pub commands: Option<Commands>,
    pub debounce: Option<u64>,
    pub log_level: Option<String>,
    pub environments: Environments,
    pub aliases: Option<Aliases>,
    pub notify_on_success: Option<bool>,
    pub notify_on_failure: Option<bool>,
    pub ignore_patterns: Option<Vec<String>>,
    pub color_output: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Commands {
    pub before: Option<String>,
    pub after: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Environments {
    pub prod: EnvironmentConfig,
    pub dev: EnvironmentConfig,
}

#[derive(Debug, Deserialize)]
pub struct EnvironmentConfig {
    pub command: String,
    pub args: Vec<String>,
    pub env: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct Aliases {
    pub build: String,
    pub test: String,
    pub lint: String,
}

pub fn parse() -> Result<Config, Box<dyn Error>> {
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
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}
