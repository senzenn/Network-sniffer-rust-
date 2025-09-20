use super::cli::Commands;
use crate::capture::{list_devices, capture_packets, analyze_packet};

pub fn handle_command(command: Commands) {
    match command {
        Commands::ListDevices => {
            list_devices();
        }
        Commands::Capture {
            device,
            promiscuous,
            snaplen,
            count,
        } => {
            capture_packets(device, promiscuous, snaplen, count);
        }
        Commands::Analyze { packet } => {
            analyze_packet(packet);
        }
    }
}
