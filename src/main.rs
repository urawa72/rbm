use std::path::PathBuf;
use rbm::config::load_config;
use structopt::StructOpt;
use rbm::bookmarks::{add_bookmark, list_bookmarks};
use rbm::cli::{Action::*, CommandLineArgs};

fn find_config_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push("rbm-config.toml");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    let config_file = find_config_file().expect("Failed to find config file");
    let journal_file = load_config(config_file).expect("Failed to load journal file");

    match action {
        Add => add_bookmark(journal_file),
        List => list_bookmarks(journal_file),
    }?;
    Ok(())
}
