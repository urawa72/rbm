use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// List all bookmarks in the journal file.
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
