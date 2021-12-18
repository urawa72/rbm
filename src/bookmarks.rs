use crate::finder::finder;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, OpenOptions},
    io::{stdin, stdout, BufRead, Error, Write},
    path::PathBuf,
};

#[derive(Debug, Deserialize)]
pub struct Bookmarks {
    bookmark: Vec<Bookmark>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bookmark {
    pub url: String,
    pub inner: String,
}

/// Add a new bookmark url and search string to the toml file.
pub fn add_bookmark(file_path: PathBuf) -> Result<(), Error> {
    let reader = stdin();
    let mut handle = reader.lock();

    let mut title = String::new();
    print!("Title> ");
    stdout().flush()?;
    handle.read_line(&mut title)?;

    let mut url = String::new();
    print!("URL> ");
    stdout().flush()?;
    handle.read_line(&mut url)?;

    let mut tags = String::new();
    print!("Tags> ");
    stdout().flush()?;
    handle.read_line(&mut tags)?;

    let tags = tags
        .trim()
        .split(',')
        .collect::<Vec<_>>()
        .iter()
        .map(|&tag| "#".to_string() + tag + " ")
        .collect::<String>();

    let bookmark = Bookmark {
        url: url.trim().to_string(),
        inner: format!("{} {}", title.trim().to_string(), tags.trim()),
    };

    let toml = toml::to_string(&bookmark);
    let toml = match toml {
        Ok(toml) => toml,
        Err(e) => panic!("Failed to parse input into toml string: {}", e),
    };
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    write!(f, "[[bookmark]]\n{}\n", toml)?;
    f.flush()?;

    Ok(())
}

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
