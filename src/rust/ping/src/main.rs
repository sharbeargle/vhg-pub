use nix::sys::socket::{socket, SockFlag, SockaddrIn, MsgFlags};
use nix::sys::socket::AddressFamily;
use nix::sys::socket::SockType;
use nix::sys::socket::sendto;
use nix::sys::socket::recvfrom;
use std::net::Ipv4Addr;
use std::os::fd::AsRawFd;
use simpleargs::{ArgType, Parser, Arg};

fn parse_args() -> Parser {
    simpleargs::new("ping".to_string())
        .add_flag(
            "dst_ip".to_string(), 
            Some("dst_ip".to_string()), 
            Some('d'), 
            true, 
            Some(ArgType::String), 
            "The target IP to ping".to_string()
        ).parse(std::env::args())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p = parse_args();

    let dst_ip = match p.get_arg("dst_ip") {
        Some(Arg::String(dst)) => dst.to_owned(),
        _ => "127.0.0.1".to_string()
    };

    let target_ip = dst_ip.parse::<Ipv4Addr>().unwrap();
    let target_ip_octets = target_ip.octets();
    let target_addr = SockaddrIn::new(target_ip_octets[0], target_ip_octets[1], target_ip_octets[2], target_ip_octets[3], 0); // ICMP doesn't use ports

    // Create a raw socket for ICMP
    let sock = socket(AddressFamily::Inet, SockType::Raw, SockFlag::empty(), None)?;

    // Construct a simple ICMP echo request
    let mut data: Vec<u8> = vec![0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    // Dummy sequence number and checksum (replace with actual values)
    data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); 

    // Send the ICMP echo request
    sendto(sock.as_raw_fd(), &data, &target_addr, MsgFlags::empty())?;

    println!("ICMP echo request sent to {}", target_ip);
    
    // Optionally receive a response (non-blocking)
    let mut buf = vec![0u8; 1024];
    match recvfrom::<SockaddrIn>(sock.as_raw_fd(), &mut buf) {
        Ok((size, _)) => {
            println!("Received {} bytes of data", size);
            // Process the received data (e.g., parse ICMP echo reply)
        }
        Err(nix::errno::Errno::EWOULDBLOCK) => {
            println!("No response received within timeout.");
        },
        Err(e) => {
            println!("Error receiving data: {:?}", e);
        }
    }

    Ok(())
}
