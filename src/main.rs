use std::path::PathBuf;
use structopt::StructOpt;
use rbm::bookmarks::{add_bookmark, list_bookmarks};
use rbm::cli::{Action::*, CommandLineArgs};

fn find_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".RustyBookmarks.toml");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    let journal_file = find_journal_file().expect("Failed to find journal file");

    match action {
        Add => add_bookmark(journal_file),
        List => list_bookmarks(journal_file),
    }?;
    Ok(())
}
