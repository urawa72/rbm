use std::io::{self, BufRead, Error, stdout, Write};

#[derive(Debug)]
pub struct Bookmark {
    title: String,
    url: String,
}

pub fn add_bookmark() -> Result<(), Error> {
    let reader = io::stdin();
    let mut handle = reader.lock();

    let mut title = String::new();
    print!("Title: ");
    stdout().flush()?;
    handle.read_line(&mut title)?;

    let mut url = String::new();
    print!("URL: ");
    stdout().flush()?;
    handle.read_line(&mut url)?;

    println!(
        "title = {} url = {}",
        title.trim().to_string(),
        url.trim().to_string()
    );
    Ok(())
}

pub fn list_bookmarks() -> Result<(), Error> {
    println!("Hello, CLI World!!!");
    Ok(())
}
