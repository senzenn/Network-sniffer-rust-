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

        /// BPF filter expression (e.g., "tcp port 80")
        #[arg(short, long)]
        filter: Option<String>,

        /// Output file to save captured packets (in pcap format)
        #[arg(short, long)]
        output_file: Option<String>,

        /// Analyze packets in real-time during capture
        #[arg(short, long)]
        realtime_analysis: bool,
    },

    Analyze {
        /// Path to a pcap file or raw packet data file
        packet_file: String,
    },
}
