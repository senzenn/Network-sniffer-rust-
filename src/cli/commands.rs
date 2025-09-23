use super::cli::Commands;
use crate::capture::{list_devices, capture_packets, analyze_packet};
use std::fs;
use anyhow::Result;

pub fn handle_command(command: Commands) {
    match command {
        Commands::ListDevices => {
            if let Err(e) = list_devices() {
                eprintln!("Error listing devices: {}", e);
            }
        }
        Commands::Capture {
            device,
            promiscuous,
            snaplen,
            count,
            filter,
            output_file,
            realtime_analysis,
        } => {
            if let Err(e) = capture_packets(device, promiscuous, snaplen, count, filter, output_file, realtime_analysis) {
                eprintln!("Error capturing packets: {}", e);
            }
        }
        Commands::Analyze { packet_file } => {
            if let Err(e) = analyze_file(&packet_file) {
                eprintln!("Error analyzing packet file: {}", e);
            }
        }
    }
}

fn analyze_file(file_path: &str) -> Result<()> {
    let file_data = fs::read(file_path)?;

    if file_data.len() >= 4 && &file_data[0..4] == &[0xD4, 0xC3, 0xB2, 0xA1] {
        analyze_pcap_file(file_path)?;
    } else {
        analyze_packet(&file_data)?;
    }

    Ok(())
}

fn analyze_pcap_file(file_path: &str) -> Result<()> {
    use pcap::Capture;

    let mut cap = Capture::from_file(file_path)?;

    let mut packet_count = 0;
    while let Ok(packet) = cap.next_packet() {
        packet_count += 1;
        println!("\n--- Analyzing packet {} ---", packet_count);
        if let Err(e) = analyze_packet(&packet.data) {
            eprintln!("Error analyzing packet {}: {}", packet_count, e);
        }
    }

    println!("\nFinished analyzing {} packets from pcap file.", packet_count);
    Ok(())
}
