use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// Define a structure that matches the expected layout of your configuration file.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub logging_level: String,
    pub network: NetworkConfig,
    pub encryption: EncryptionConfig,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub tunneling_protocol: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct EncryptionConfig {
    pub method: String,
    pub key: String,
}


pub fn parse_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    // Open the file in read-only mode.
    let mut file = File::open(path)?;

    // Read the contents of the file into a string.
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the TOML string into a Config object.
    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}

