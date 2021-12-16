use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
    path::PathBuf,
};

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub path: PathBuf,
}

pub fn find_config_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push("rbm-config.toml");
        path
    })
}

pub fn load_config(config_file_path: PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let toml = read_to_string(config_file_path.clone());
    let toml = match toml {
        Ok(toml) => toml,
        Err(_) => {
            // Create new config file if not exist
            let journal_file_path = find_journal_file().expect("Failed to find journal file");
            let config = Config { path: journal_file_path };
            let config_file = ConfigFile { config };
            let toml = toml::to_string(&config_file).expect("Failed to to_string config");
            let mut f = OpenOptions::new()
                .create(true)
                .write(true)
                .open(config_file_path)?;
            write!(f, "{}", toml.clone())?;
            toml
        }
    };
    let config_file: Result<ConfigFile, toml::de::Error> = toml::from_str(&toml);
    match config_file {
        Ok(config_file) => Ok(config_file.config),
        Err(e) => panic!("Failed to load toml: {}", e),
    }
}

fn find_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push("rbm-bookmarks.toml");
        path
    })
}

