use crate::cli::{add, find, test_db};
use clap::{ArgAction, Parser, Subcommand};

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

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "Add a new register to the score table",
        subcommand_required = true
    )]
    Add {
        #[command(subcommand)]
        command: Option<add::Add>,
    },
    #[command(
        about = "Just testing DB",
        //subcommand_required = true
    )]
    TestDB {},
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { command, .. }) => {
            add::handle(command.clone());
        }
        Some(Commands::TestDB { .. }) => {
            test_db::handle();
        }
        None => {
            // Filtrar caminhos excluídos
            if !cli.exclude.is_empty() {
                let _ = find::handle(cli.exclude);
                return;
            }

            let _ = find::handle(Vec::new());
        }
    }
}
