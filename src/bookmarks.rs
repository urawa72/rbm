use serde::{Deserialize, Serialize};
use core::fmt;
use std::{
    fs::{File, OpenOptions},
    io::{stdin, stdout, BufRead, Error, Seek, SeekFrom, Write},
    path::PathBuf,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Bookmark {
    title: String,
    url: String,
}

impl fmt::Display for Bookmark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]({})", self.title, self.url)
    }
}

fn collect_bookmarks(mut file: &File) -> Result<Vec<Bookmark>, Error> {
    file.seek(SeekFrom::Start(0))?;
    let bookmarks: Vec<Bookmark> = match serde_json::from_reader(file) {
        Ok(bookmarks) => bookmarks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(bookmarks)
}

pub fn add_bookmark(journal_file: PathBuf) -> Result<(), Error> {
    let reader = stdin();
    let mut handle = reader.lock();

    let mut title = String::new();
    print!("Title: ");
    stdout().flush()?;
    handle.read_line(&mut title)?;

    let mut url = String::new();
    print!("URL: ");
    stdout().flush()?;
    handle.read_line(&mut url)?;

    let bookmark = Bookmark {
        title: title.trim().to_string(),
        url: url.trim().to_string()
    };

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_file)?;

    let mut bookmarks = collect_bookmarks(&file)?;
    bookmarks.push(bookmark);
    serde_json::to_writer(file, &bookmarks)?;

    Ok(())
}

pub fn list_bookmarks(journal_file: PathBuf) -> Result<(), Error> {
    let file = OpenOptions::new().read(true).open(journal_file)?;
    let bookmarks = collect_bookmarks(&file)?;
    if bookmarks.is_empty() {
        println!("Bookmark list is empty!");
    } else {
        let mut order: u32 = 1;
        for bookmark in bookmarks.iter() {
            println!("{}: {}", order, bookmark);
            order += 1;
        }
    }
    Ok(())
}
