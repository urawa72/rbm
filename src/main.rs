mod bookmark;
mod cli;
mod cmd;
mod config;
mod finder;

use cli::{Action::*, CommandLineArgs};
use cmd::add::add_bookmark;
use cmd::list::list_bookmarks;
use config::load_config;
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
