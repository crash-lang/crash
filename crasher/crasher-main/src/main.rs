use clap::{Parser};
use crate::command::Cli;

mod command;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
}