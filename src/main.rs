use std::path::PathBuf;

use cli::{Action::*, CommandLineArgs};
use structopt::StructOpt;

mod bookmarks;
mod cli;

fn find_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-bookmarks.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    let journal_file = find_journal_file().expect("Failed to find journal file");

    match action {
        Add => bookmarks::add_bookmark(journal_file),
        List => bookmarks::list_bookmarks(journal_file),
    }?;
    Ok(())
}
