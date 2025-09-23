use anyhow::Result;
use pcap::{Device, Capture, Savefile};

pub fn list_devices() -> Result<()> {
    let devices = Device::list()?;
    if devices.is_empty() {
        println!("No network devices found.");
        return Ok(());
    }

    println!("Available network devices:");
    for (i, device) in devices.iter().enumerate() {
        println!("{}. {} - {}", i + 1, device.name, device.desc.as_deref().unwrap_or("No description"));
    }
    Ok(())
}

pub fn capture_packets(device: String, promiscuous: bool, snaplen: usize, count: usize, filter: Option<String>, output_file: Option<String>, realtime_analysis: bool) -> Result<()> {
    let cap = Capture::from_device(device.as_str())?
        .promisc(promiscuous)
        .snaplen(snaplen as i32);

    let mut cap = cap.open()?;

    // Note: BPF filter application may require a different approach in pcap crate
    if let Some(_filter_expr) = &filter {
        println!("BPF filter specified: {} (filter application not implemented yet)", filter.as_ref().unwrap());
    }

    let mut savefile: Option<Savefile> = if let Some(file) = output_file {
        Some(cap.savefile(&file)?)
    } else {
        None
    };

    println!("Starting packet capture on device: {} (promiscuous: {}, snaplen: {}, count: {})",
             device, promiscuous, snaplen, count);

    let mut packet_count = 0;
    let target_count = if count == 0 { usize::MAX } else { count };

    while packet_count < target_count {
        match cap.next_packet() {
            Ok(packet) => {
                packet_count += 1;
                println!("Captured packet {}: {} bytes", packet_count, packet.header.len);

                if let Some(ref mut sf) = savefile {
                    sf.write(&packet);
                }

                // Analyze packet in real-time if requested
                if realtime_analysis {
                    println!("\n--- Real-time Analysis of Packet {} ---", packet_count);
                    if let Err(e) = analyze_packet(&packet.data) {
                        eprintln!("Error analyzing packet {}: {}", packet_count, e);
                    }
                    println!();
                }
            }
            Err(e) => {
                eprintln!("Error capturing packet: {}", e);
                break;
            }
        }
    }

    println!("Capture finished. Total packets: {}", packet_count);
    Ok(())
}

pub fn analyze_packet(packet: &[u8]) -> Result<()> {
    use etherparse::{Ipv4HeaderSlice, Ipv6HeaderSlice, TcpHeaderSlice, UdpHeaderSlice, Icmpv4Slice, Icmpv6Slice};

    println!("Analyzing packet ({} bytes)", packet.len());

    if let Ok(ipv4_slice) = Ipv4HeaderSlice::from_slice(packet) {
        println!("IPv4 Packet:");
        println!("  Source IP: {}", ipv4_slice.source_addr());
        println!("  Destination IP: {}", ipv4_slice.destination_addr());
        println!("  Protocol: {}", ipv4_slice.protocol());
        println!("  TTL: {}", ipv4_slice.ttl());
        println!("  Total Length: {}", ipv4_slice.total_len());

        let ip_payload = &packet[ipv4_slice.slice().len()..];

        match ipv4_slice.protocol() {
            6 => {
                if let Ok(tcp_slice) = TcpHeaderSlice::from_slice(ip_payload) {
                    println!("  TCP Header:");
                    println!("    Source Port: {}", tcp_slice.source_port());
                    println!("    Destination Port: {}", tcp_slice.destination_port());
                    println!("    Sequence Number: {}", tcp_slice.sequence_number());
                    println!("    Acknowledgment Number: {}", tcp_slice.acknowledgment_number());
                    println!("    Flags: SYN={}, ACK={}, FIN={}, RST={}, PSH={}, URG={}",
                             tcp_slice.syn(), tcp_slice.ack(), tcp_slice.fin(), tcp_slice.rst(), tcp_slice.psh(), tcp_slice.urg());
                } else {
                    eprintln!("Failed to parse TCP header");
                }
            }
            17 => {
                if let Ok(udp_slice) = UdpHeaderSlice::from_slice(ip_payload) {
                    println!("  UDP Header:");
                    println!("    Source Port: {}", udp_slice.source_port());
                    println!("    Destination Port: {}", udp_slice.destination_port());
                    println!("    Length: {}", udp_slice.length());
                } else {
                    eprintln!("Failed to parse UDP header");
                }
            }
                        1 => {
                            if let Ok(icmp_slice) = Icmpv4Slice::from_slice(ip_payload) {
                                println!("  ICMP Header:");
                                println!("    Type: {:?}", icmp_slice.icmp_type());
                            } else {
                                eprintln!("Failed to parse ICMP header");
                            }
                        }
            _ => {
                println!("  Unknown or unsupported IPv4 protocol");
            }
        }
    } else if let Ok(ipv6_slice) = Ipv6HeaderSlice::from_slice(packet) {
        println!("IPv6 Packet:");
        println!("  Source IP: {}", ipv6_slice.source_addr());
        println!("  Destination IP: {}", ipv6_slice.destination_addr());
        println!("  Next Header: {}", ipv6_slice.next_header());
        println!("  Hop Limit: {}", ipv6_slice.hop_limit());
        println!("  Payload Length: {}", ipv6_slice.payload_length());

        let ip_payload = &packet[ipv6_slice.slice().len()..];

        match ipv6_slice.next_header() {
            6 => {
                if let Ok(tcp_slice) = TcpHeaderSlice::from_slice(ip_payload) {
                    println!("  TCP Header:");
                    println!("    Source Port: {}", tcp_slice.source_port());
                    println!("    Destination Port: {}", tcp_slice.destination_port());
                    println!("    Sequence Number: {}", tcp_slice.sequence_number());
                    println!("    Acknowledgment Number: {}", tcp_slice.acknowledgment_number());
                    println!("    Flags: SYN={}, ACK={}, FIN={}, RST={}, PSH={}, URG={}",
                             tcp_slice.syn(), tcp_slice.ack(), tcp_slice.fin(), tcp_slice.rst(), tcp_slice.psh(), tcp_slice.urg());
                } else {
                    eprintln!("Failed to parse TCP header");
                }
            }
            17 => {
                if let Ok(udp_slice) = UdpHeaderSlice::from_slice(ip_payload) {
                    println!("  UDP Header:");
                    println!("    Source Port: {}", udp_slice.source_port());
                    println!("    Destination Port: {}", udp_slice.destination_port());
                    println!("    Length: {}", udp_slice.length());
                } else {
                    eprintln!("Failed to parse UDP header");
                }
            }
                        58 => {
                            if let Ok(icmpv6_slice) = Icmpv6Slice::from_slice(ip_payload) {
                                println!("  ICMPv6 Header:");
                                println!("    Type: {:?}", icmpv6_slice.icmp_type());
                            } else {
                                eprintln!("Failed to parse ICMPv6 header");
                            }
                        }
            _ => {
                println!("  Unknown or unsupported IPv6 protocol");
            }
        }
    } else {
        println!("Not an IP packet or failed to parse");
        print_hex_dump(packet);
    }

    Ok(())
}

fn print_hex_dump(data: &[u8]) {
    const BYTES_PER_LINE: usize = 16;

    for (i, chunk) in data.chunks(BYTES_PER_LINE).enumerate() {
        print!("{:08x}: ", i * BYTES_PER_LINE);

        for byte in chunk {
            print!("{:02x} ", byte);
        }

        for _ in 0..(BYTES_PER_LINE - chunk.len()) {
            print!("   ");
        }

        print!(" ");

        for byte in chunk {
            if *byte >= 32 && *byte <= 126 {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }

        println!();
    }
}
