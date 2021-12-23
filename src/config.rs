use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
    path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub path: PathBuf,
}

/// Load the config file.
/// If not exist, create a new config file in the home directory.
pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_file_path = find_config_file().expect("Failed to find the config file");
    let toml = read_to_string(config_file_path.clone());
    let toml = match toml {
        Ok(toml) => toml,
        Err(_) => {
            // Create new config file if not exist
            let bookmarks_file_path =
                find_bookmarks_file().expect("Failed to find the bookmarks file");
            let config = Config {
                path: bookmarks_file_path,
            };
            let config_file = ConfigFile { config };
            let toml = toml::to_string(&config_file).expect("Failed to parse into toml string");
            let mut f = OpenOptions::new()
                .create(true)
                .write(true)
                .open(config_file_path)?;
            write!(f, "{}", toml)?;
            toml
        }
    };

    // Load config file content
    let config_file: Result<ConfigFile, toml::de::Error> = toml::from_str(&toml);
    match config_file {
        Ok(config_file) => Ok(config_file.config),
        Err(e) => panic!("Failed to load the config toml: {}", e),
    }
}

/// Find config file path
fn find_config_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push("rbm-config.toml");
        path
    })
}

/// Find bookmarks file path
fn find_bookmarks_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push("rbm-bookmarks.toml");
        path
    })
}
