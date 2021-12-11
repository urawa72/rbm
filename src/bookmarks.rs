extern crate skim;
use serde::{Deserialize, Serialize};
use skim::prelude::*;
use skim::{Skim, SkimItem, SkimItemReceiver, SkimItemSender};
use std::{
    borrow::Cow,
    fs::{File, OpenOptions},
    io::{stdin, stdout, BufRead, Error, Seek, SeekFrom, Write},
    path::PathBuf,
    process::Command,
    sync::Arc,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Bookmark {
    url: String,
    inner: String
}

impl SkimItem for Bookmark {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.inner)
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.url)
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
        inner: title.trim().to_string() + " ["+ tags.trim() + "]",
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
        return Ok(());
    }

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .bind(vec!["Enter:accept"])
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for bookmark in bookmarks {
        let _ = tx_item.send(Arc::new(bookmark));
    }
    drop(tx_item);

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| match out.final_key {
            Key::Enter => out.selected_items,
            _ => Vec::new(),
        })
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        let url = item.output();
        Command::new("open")
            .arg(url.as_ref())
            .output()
            .expect("faild to execute process");
    }

    Ok(())
}
