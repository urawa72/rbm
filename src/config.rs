use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
    path::PathBuf,
};

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    path: PathBuf,
}

fn find_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push("rbm-bookmarks.toml");
        path
    })
}

pub fn load_config(config_file: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let toml = read_to_string(config_file.clone());
    let toml = match toml {
        Ok(toml) => toml,
        Err(_) => {
            let journal_file = find_journal_file().expect("Failed to find journal file");
            let config = Config { path: journal_file };
            let toml = toml::to_string(&config).expect("Failed to to_string config");
            let mut f = OpenOptions::new()
                .create(true)
                .write(true)
                .open(config_file)?;
            write!(f, "[config]\n{}", toml.clone())?;
            toml
        }
    };
    let config: Result<Config, toml::de::Error> = toml::from_str(&toml);
    match config {
        Ok(config) => Ok(config.path),
        Err(e) => panic!("Failed to load toml: {}", e),
    }
}
