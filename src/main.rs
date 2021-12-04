use cli::CommandLineArgs;
use structopt::StructOpt;

mod cli;

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    match action {
        _ => println!("Hello, CLI World!!!"),
    };
    Ok(())
}
