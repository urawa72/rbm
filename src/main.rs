use rbm::config::{find_config_file, load_config};
use structopt::StructOpt;
use rbm::bookmarks::{add_bookmark, list_bookmarks};
use rbm::cli::{Action::*, CommandLineArgs};

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    let config_file_path = find_config_file().expect("Failed to find config file");
    let config = load_config(config_file_path).expect("Failed to load config file");

    match action {
        Add => add_bookmark(config.path),
        List => list_bookmarks(config.path),
    }?;
    Ok(())
}
