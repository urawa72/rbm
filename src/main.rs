use cli::{Action::*, CommandLineArgs};
use structopt::StructOpt;

mod bookmarks;
mod cli;

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    match action {
        Add  => bookmarks::add_bookmark(),
        List => bookmarks::list_bookmarks(),
    }?;
    Ok(())
}
