use crate::cli::find;
use clap::{ArgAction, Parser};

#[derive(Parser)]
//#[command(version, about = "Super find things!", long_about = None, arg_required_else_help = true)]
#[command(
    version,
    about = "Super find things!",
    long_about = "No, really... you super find things with this tool!"
)]
struct Cli {
    /// List of paths to exclude from the search
    #[arg(short = 'e', long = "exclude", action = ArgAction::Append)]
    exclude: Vec<String>,
}

pub fn run() {
    let cli = Cli::parse();
    let exclude = if !cli.exclude.is_empty() { cli.exclude } else { Vec::new() };
    let _ = find::handle(exclude);
}
