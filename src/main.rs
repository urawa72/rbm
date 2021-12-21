use rbm::cli::{Action::*, CommandLineArgs};
use rbm::config::load_config;
use structopt::StructOpt;

mod cmd;

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    let config = load_config().expect("Failed to load the config file");

    match action {
        Add => cmd::add::add_bookmark(config.path),
        List => cmd::list::list_bookmarks(config.path),
    }?;
    Ok(())
}
