use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rust-one")]
#[command(about = "A CLI tool for network packet capture and analysis")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List available network devices
    ListDevices,

    Capture {
        #[arg(short, long, default_value = "en0")]
        device: String,

        #[arg(short, long)]
        promiscuous: bool,

        #[arg(short, long, default_value = "5000")]
        snaplen: usize,

        #[arg(short = 'n', long, default_value = "0")]
        count: usize,
    },

    Analyze {
        packet: String,
    },
}
