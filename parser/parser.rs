use serde::Deserialize;
use serde_yaml;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    version: f32,
    engines: Engines,
    index: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct Engines {
    runtime: String,
    #[serde(rename = "runtime-devel")]
    runtime_devel: String,
}

pub fn parse() -> Result<(), Box<dyn Error>> {
    let file_path = "watch.yml";
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_yaml::from_str(&contents)?;

    println!("{:#?}", config);

    Ok(())
}
