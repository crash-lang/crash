use clap::Parser;
use crate::command::Cli;

mod command;
mod styles;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
}