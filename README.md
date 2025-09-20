# Rust Network Sniffer

A high-performance network packet capture and analysis tool built in Rust, designed for network monitoring, debugging, and security analysis.

## Features

- **Multi-platform Support**: Works on Linux, macOS, and Windows
- **Real-time Packet Capture**: Capture packets from network interfaces in real-time
- **Device Discovery**: Automatically detect and list available network devices
- **Promiscuous Mode**: Capture all network traffic on a device
- **Packet Analysis**: Basic packet parsing and inspection capabilities
- **Configurable Capture**: Set snapshot length, packet count limits, and capture parameters
- **CLI Interface**: Command-line interface with subcommands for different operations

## Installation

### Prerequisites

- Rust 1.70 or later
- libpcap development headers (install via your package manager)

### Building from Source

```bash
git clone <repository-url>
cd rust-network-sniffer
cargo build --release
```

The compiled binary will be available at `target/release/rust-one`.

## Usage

### List Available Network Devices

```bash
./rust-one list-devices
```

### Capture Packets

```bash
# Capture packets on default device (en0) with default settings
./rust-one capture

# Capture on specific device with custom settings
./rust-one capture --device eth0 --promiscuous --snaplen 1500 --count 100

# Options:
#   -d, --device <DEVICE>      Network device to capture on [default: en0]
#   -p, --promiscuous          Enable promiscuous mode
#   -s, --snaplen <SNAPLEN>    Snapshot length (bytes) [default: 5000]
#   -n, --count <COUNT>        Number of packets to capture [default: 0 (unlimited)]
```

### Analyze Packets

```bash
./rust-one analyze --packet "<packet_data>"
```

### Help

```bash
./rust-one --help
./rust-one <subcommand> --help
```

## Extensions and Improvements

This is just a starting point. Here are some ways you could extend this tool:

### Protocol Decoders
- **HTTP Parser**: Decode HTTP requests and responses with headers, methods, and status codes
- **DNS Resolver**: Parse DNS queries and responses with domain names and record types
- **TLS/SSL Inspector**: Analyze encrypted traffic patterns and certificate information
- **Database Protocol Support**: Decode MySQL, PostgreSQL, and MongoDB protocol packets

### Advanced Filtering
- **BPF (Berkeley Packet Filter)**: Implement low-level packet filtering with BPF expressions
- **Smart Filtering**: Host-based, port-based, and protocol-based filtering
- **Content Filtering**: Search for specific strings or patterns in packet payloads
- **Time-based Filtering**: Capture packets within specific time windows

### Statistics and Analytics
- **Traffic Statistics**: Real-time bandwidth usage, packet rates, and protocol distribution
- **Connection Tracking**: Monitor TCP connection states and session information
- **Geolocation**: Map IP addresses to geographical locations
- **Anomaly Detection**: Identify unusual traffic patterns or potential security threats

### Data Persistence
- **PCAP File Support**: Save captured packets to standard PCAP format for Wireshark analysis
- **Database Storage**: Store packet metadata and analysis results in SQLite/PostgreSQL
- **Export Formats**: Support for JSON, CSV, and XML export of captured data
- **Packet Replay**: Ability to replay captured traffic for testing

### User Interfaces
- **Terminal User Interface (TUI)**: Rich text-based interface with live packet display and filtering
- **Web Interface**: Browser-based dashboard for remote monitoring and analysis
- **Graphical Interface**: Native desktop application with packet visualization
- **REST API**: HTTP API for integration with other tools and automation

### Performance and Scalability
- **Multi-threading**: Parallel packet processing for high-throughput networks
- **GPU Acceleration**: Hardware-accelerated packet analysis for massive traffic
- **Distributed Capture**: Coordinate multiple capture agents across network segments
- **Cloud Integration**: Store and analyze captures in cloud storage with serverless processing

### Security Features
- **Intrusion Detection**: Pattern matching against known attack signatures
- **Privacy Protection**: Automatic masking of sensitive data in captures
- **Compliance Tools**: Generate reports for regulatory compliance (PCI-DSS, HIPAA)
- **Forensic Analysis**: Timeline reconstruction and evidence preservation

### Additional Protocols
- **VoIP Analysis**: SIP, RTP, and media stream analysis
- **IoT Protocols**: MQTT, CoAP, and other IoT communication protocols
- **Industrial Control**: Modbus, DNP3, and SCADA protocol support
- **Wireless Networks**: WiFi frame analysis and 802.11 protocol decoding

## Architecture

The tool is built with a modular architecture:

- `cli/`: Command-line interface and argument parsing
- `capture/`: Core packet capture functionality using libpcap
- `analysis/`: Packet parsing and protocol analysis

### System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    Rust Network Sniffer                         │
│                    ┌─────────────────────────────────┐          │
│                    │         CLI Layer              │          │
│                    │  ┌─────────────────────────┐   │          │
│                    │  │    Command Parser       │   │          │
│                    │  │    (clap crate)         │   │          │
│                    │  └─────────────────────────┘   │          │
│                    │  ┌─────────────────────────┐   │          │
│                    │  │   Command Handler       │   │          │
│                    │  │   (commands.rs)         │   │          │
│                    │  └─────────────────────────┘   │          │
│                    └─────────────────────────────────┘          │
│           ┌─────────────────────────────────────────────────┐   │
│           │              Capture Layer                      │   │
│           │  ┌─────────────────┐  ┌─────────────────────┐   │   │
│           │  │  Device Mgmt    │  │   Packet Capture     │   │   │
│           │  │  (list_devices) │  │  (capture_packets)   │   │   │
│           │  └─────────────────┘  └─────────────────────┘   │   │
│           │  ┌─────────────────┐  ┌─────────────────────┐   │   │
│           │  │  Packet Analysis │  │   Network Interface │   │   │
│           │  │ (analyze_packet) │  │     (libpcap)       │   │   │
│           │  └─────────────────┘  └─────────────────────┘   │   │
│           └─────────────────────────────────────────────────┘   │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │                 External Dependencies                       │ │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │ │
│  │  │   libpcap   │  │  etherparse │  │    clap     │          │ │
│  │  │ (capture)   │  │ (parsing)   │  │   (CLI)     │          │ │
│  │  └─────────────┘  └─────────────┘  └─────────────┘          │ │
│  └─────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Command Flow Diagram

```
User Input
    │
    ▼
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   clap Parser   │────▶│  Command Match   │────▶│  Handler Call   │
│   (Cli::parse)  │     │   (match cmd)    │     │ (handle_command)│
└─────────────────┘     └──────────────────┘     └─────────────────┘
         │                        │                        │
         │                        │                        │
         ▼                        ▼                        ▼
    ListDevices ────────────▶ list_devices() ────────▶ Display devices
                              capture/::list_devices

    Capture ───────────────▶ capture_packets() ────▶ Start packet capture
    (device, promiscuous,   capture/::capture_packets
     snaplen, count)

    Analyze ──────────────▶ analyze_packet() ─────▶ Parse packet data
    (packet)               capture/::analyze_packet
```

### Packet Capture Flow

```
Network Traffic
       │
       │ (Raw packets)
       ▼
┌─────────────────┐
│ Network Device  │ (eth0, wlan0, etc.)
│   (Interface)   │
└─────────────────┘
       │
       │ (Packet stream)
       ▼
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   libpcap       │────▶│  Packet Filter   │────▶│  Packet Buffer  │
│   Capture       │     │  (BPF/Filter)    │     │  (Raw bytes)    │
└─────────────────┘     └──────────────────┘     └─────────────────┘
       │                        │                        │
       │                        │                        │
       ▼                        ▼                        ▼
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Packet Parser  │────▶│ Protocol Decoder │────▶│   Analysis      │
│ (etherparse)    │     │ (TCP/IP/UDP)     │     │   Results       │
└─────────────────┘     └──────────────────┘     └─────────────────┘
       │
       │ (Parsed data)
       ▼
   User Display
   (Terminal/Output)
```

### Module Dependency Graph

```
src/main.rs
    │
    ├── mod cli;
    │   ├── mod cli.rs (Cli struct, Commands enum)
    │   ├── mod commands.rs (handle_command function)
    │   └── mod.rs (module exports)
    │
    └── mod capture;
        ├── mod capture.rs (core functions)
        └── mod.rs (module exports)
```

### Data Flow Summary

1. **Command Parsing**: User input → clap parser → Commands enum → handle_command()
2. **Device Discovery**: handle_command() → capture::list_devices() → libpcap device enumeration
3. **Packet Capture**: handle_command() → capture::capture_packets() → libpcap capture loop
4. **Packet Analysis**: handle_command() → capture::analyze_packet() → etherparse packet parsing

### Configuration Parameters

```
CLI Arguments → Capture Parameters
├── device: String     → Interface selection (eth0, wlan0, etc.)
├── promiscuous: bool  → Promiscuous mode (capture all packets)
├── snaplen: usize     → Packet snapshot length (bytes to capture)
└── count: usize       → Maximum packet count (0 = unlimited)
```

## Dependencies

- `pcap`: Low-level packet capture library
- `etherparse`: Ethernet frame and packet parsing
- `clap`: Command-line argument parser
- `anyhow`: Error handling

## Contributing

Contributions are welcome! Please feel free to submit pull requests, report issues, or suggest new features.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
