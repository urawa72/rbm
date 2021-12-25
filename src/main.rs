mod cli;
mod config;
mod cmd;
mod bookmark;
mod finder;

use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use config::load_config;
use cmd::add::add_bookmark;
use cmd::list::list_bookmarks;

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    let config = load_config().expect("Failed to load the config file");

    match action {
        Add => add_bookmark(config.path),
        List => list_bookmarks(config.path),
    }?;
    Ok(())
}
