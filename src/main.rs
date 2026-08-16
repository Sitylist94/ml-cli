mod cli;
mod commands;
mod config;
mod generator;
mod templates;

fn main() -> anyhow::Result<()> {
    cli::run()
}
