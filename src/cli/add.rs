use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub enum Add {
    #[command(about = "The register to add")]
    Register {},
}

pub fn handle(command: Option<Add>) {
    if let Some(subcommand) = command {
        match subcommand {
            Add::Register { .. } => println!("New register added to score table!"),
        }
    }
}
