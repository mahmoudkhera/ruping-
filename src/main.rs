use std::net::Ipv4Addr;

use ruping::ip_data::{IpProtocol, RawIpv4};

fn main() {
    let src = Ipv4Addr::new(192, 168, 1, 100);
    let dst = Ipv4Addr::new(8, 8, 8, 8);

    let mut ipv4 = RawIpv4::new(0, 200, src, dst);

    let data = [1, 2];

    let ip4_packet = ipv4.evaluate_ipv4(IpProtocol::ICMP, &data);


    println!("IPv4 Packet ({} bytes):", ip4_packet.len());
    for (i, b) in ip4_packet.iter().enumerate() {
        print!("{:02X} ", b);
        if (i + 1) % 8 == 0 {
            println!();
        }
    }
    println!();
}
