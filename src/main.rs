use rbm::bookmarks::{add_bookmark, list_bookmarks};
use rbm::cli::{Action::*, CommandLineArgs};
use rbm::config::load_config;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    let config = load_config().expect("Failed to load the config file");

    match action {
        Add => add_bookmark(config.path),
        List => list_bookmarks(config.path),
    }?;
    Ok(())
}
