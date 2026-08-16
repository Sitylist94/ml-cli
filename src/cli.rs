use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add,
    Remove,
    Validate,
    Doctor,
    Template,
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            crate::commands::init::run()?;
        }

        Commands::Add => {
            todo!()
        }

        Commands::Remove => {
            todo!()
        }

        Commands::Validate => {
            todo!()
        }

        Commands::Doctor => {
            todo!()
        }

        Commands::Template => {
            todo!()
        }
    }

    Ok(())
}
