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

pub fn add_bookmark(journal_file: PathBuf) -> Result<(), Error> {
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

    let bookmark = Bookmark {
        url: url.trim().to_string(),
        inner: title.trim().to_string() + " [" + tags.trim() + "]",
    };

    let toml = toml::to_string(&bookmark);
    let toml = match toml {
        Ok(toml) => toml,
        Err(e) => panic!("Failed to parse toml: {}", e),
    };
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(journal_file)?;
    write!(f, "[[bookmark]]\n{}\n", toml)?;
    f.flush()?;

    Ok(())
}

pub fn list_bookmarks(journal_file: PathBuf) -> Result<(), Error> {
    let toml = read_to_string(journal_file)?;
    let bookmarks: Result<Bookmarks, toml::de::Error> = toml::from_str(&toml);
    let bookmarks = match bookmarks {
        Ok(bookmarks) => bookmarks,
        Err(e) => panic!("Failed to load toml: {}", e),
    };
    finder(bookmarks.bookmark);

    Ok(())
}
