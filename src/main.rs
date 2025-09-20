mod cli;
mod capture;

use clap::Parser;
use cli::{Cli, handle_command};

fn main() {
    let cli = Cli::try_parse().unwrap();
    handle_command(cli.command);
}
