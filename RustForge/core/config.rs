use std::fs::File;
use std::io::{self, Read};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub version: String,
    pub port: u16,
}

impl Config {
    pub fn new(app_name: &str, version: &str, port: u16) -> Self {
        Config {
            app_name: app_name.to_string(),
            version: version.to_string(),
            port,
        }
    }

    pub fn load_from_file(path: &str) -> Result<Self, io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}
