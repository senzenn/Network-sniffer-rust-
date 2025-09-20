use clap::{Parser, Subcommand};
use pcap::{Capture, Device};

#[derive(Parser)]
#[command(name = "rust-one")]
#[command(about = "A CLI tool for network packet capture and analysis")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::ListDevices => {
            list_devices();
        }
        Commands::Capture { device, promiscuous, snaplen, count } => {
            capture_packets(&device, promiscuous, snaplen, count);
        }
        Commands::Analyze { packet } => {
            println!("Analyzing packet: {}", packet);

            // analyze_packet(&packet);
        }
    }
}

fn list_devices() {
    println!("Available network devices:");
    println!("{:<15} {}", "Device", "Description");
    println!("{:-<15} {:-<50}", "", "");

    match Device::list() {
        Ok(devices) => {
            for device in devices {
                let desc = device.desc.unwrap_or_else(|| "No description".to_string());
                println!("{:<15} {}", device.name, desc);
            }
        }
        Err(e) => {
            eprintln!("Error listing devices: {}", e);
            std::process::exit(1);
        }
    }
}

fn capture_packets(device_name: &str, promiscuous: bool, snaplen: usize, count: usize) {
    println!("Starting packet capture on device: {}", device_name);
    println!("Promiscuous mode: {}", promiscuous);
    println!("Snapshot length: {} bytes", snaplen);
    if count > 0 {
        println!("Capturing {} packets", count);
    } else {
        println!("Capturing packets indefinitely (Ctrl+C to stop)");
    }

    let capture = match Capture::from_device(device_name) {
        Ok(cap) => cap,
        Err(e) => {
            eprintln!("Error creating capture from device '{}': {}", device_name, e);
            eprintln!("Use 'rust-one list-devices' to see available devices.");
            std::process::exit(1);
        }
    };

    let mut capture = match capture
        .promisc(promiscuous)
        .snaplen(snaplen as i32)
        .open() {
        Ok(cap) => cap,
        Err(e) => {
            eprintln!("Error opening capture: {}", e);
            eprintln!("This might require elevated permissions. Try running with sudo.");
            std::process::exit(1);
        }
    };

    let mut packet_count = 0;
    while let Ok(packet) = capture.next_packet() {
        packet_count += 1;

        println!("\n--- Packet #{} ---", packet_count);
        println!("Timestamp: {}.{}", packet.header.ts.tv_sec, packet.header.ts.tv_usec);
        println!("Captured length: {} bytes", packet.header.caplen);
        println!("Original length: {} bytes", packet.header.len);

        // Print first 64 bytes of packet data in hex
        let data_len = std::cmp::min(64, packet.data.len());
        print!("Data (first {} bytes): ", data_len);
        for (i, &byte) in packet.data.iter().enumerate().take(data_len) {
            if i > 0 && i % 16 == 0 {
                print!("\n{:24}", "");
            }
            print!("{:02x} ", byte);
        }
        println!();

        if count > 0 && packet_count >= count {
            println!("\nCaptured {} packets as requested.", count);
            break;
        }
    }

    println!("\nPacket capture completed. Total packets: {}", packet_count);
}
// basic boilerplate