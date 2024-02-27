use {
    crashc_main::commands::styles::get_styles,
    clap::{Parser, Subcommand}
};

#[derive(Parser, Debug)]
#[command(
name = "Crasher",
author,
version,
styles=get_styles()
)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Don't print Crasher log messages
    #[arg(short, long, default_value = "false")]
    quiet: bool
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    Clean {

    }
}