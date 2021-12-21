use rbm::bookmarks::Bookmarks;
use rbm::finder::finder;
use std::{fs::read_to_string, io::Error, path::PathBuf};

/// List all bookmarks from the toml file, and launch fuzzy finder.
pub fn list_bookmarks(file_path: PathBuf) -> Result<(), Error> {
    if !file_path.exists() {
        println!("Bookmarks don't exist. Add a new bookmark with the command `rbm add`.");
    } else {
        let toml = read_to_string(file_path)?;
        let bookmarks: Result<Bookmarks, toml::de::Error> = toml::from_str(&toml);
        let bookmarks = match bookmarks {
            Ok(bookmarks) => bookmarks,
            Err(e) => panic!("Failed to load the bookmarks file: {}", e),
        };
        finder(bookmarks.bookmark);
    }
    Ok(())
}
