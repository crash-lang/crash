use {
    crate::styles::get_styles,
    clap::{Parser, Subcommand}
};

#[derive(Parser, Debug)]
#[command(
name = "Crash",
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
    Build {
        #[arg(short, long)]
        input_file: String,

        #[arg(short, long)]
        output_file: String
    }
}