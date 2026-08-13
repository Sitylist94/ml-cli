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

pub fn run() {
    let _cli = Cli::parse();
}
