use structopt::StructOpt;
mod cli;
use cli::{Action::*, CommandLineArgs};
mod config;
use config::load_config;
mod cmd;
use cmd::add::add_bookmark;
use cmd::list::list_bookmarks;
mod bookmark;
mod finder;

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    let config = load_config().expect("Failed to load the config file");

    match action {
        Add => add_bookmark(config.path),
        List => list_bookmarks(config.path),
    }?;
    Ok(())
}
