use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Add bookmark to the bookmakrs file(toml).
    Add,
    /// List all bookmarks in the bookmarks file(toml).
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Bookmark Manager",
    about = "A command line bookmark management app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
