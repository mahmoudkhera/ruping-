use ruping::icmp::RawICMP;
use ruping::ip_data::IpProtocol;
use ruping::ip_data::RawIpv4;
use ruping::socket::create_sock;
use ruping::socket::send_ipv4;
use std::io;
use std::net::Ipv4Addr;

fn main() -> Result<(), io::Error> {
    let fd = create_sock().unwrap();

    // Build IP header + ICMP echo request
    // let mut packet: Vec<u8> = vec![
    //     0x45, 0x00, 0x00, 0x1c, // version/IHL, TOS, total length = 28
    //     0x00, 0x01, 0x00, 0x00, // ID, flags/frag
    //     0x40, 0x01, 0x00, 0x00, // TTL, protocol=1(ICMP), checksum placeholder
    //     127, 0, 0, 1, // src
    //     127, 0, 0, 1, // dst
    //     0x08, 0x00, 0x00, 0x00, // ICMP type=8, code=0, checksum placeholder
    //     0x12, 0x34, 0x00, 0x01, // id, seq
    // ];
    let mut icmp = RawICMP::new(ruping::icmp::KIND::ECHO);
    let data = b"echo";

    let src = Ipv4Addr::new(127, 0, 0, 1);
    let dst = Ipv4Addr::new(192, 168, 1, 6);

    let icmp = icmp.evaluate_icmp(data);

    let mut ipv4 = RawIpv4::new(0, 222, src, dst);

    let ip4_packet = ipv4.evaluate_ipv4(IpProtocol::ICMP, &icmp);
    print_hex(&ip4_packet);


    println!("{:?}", send_ipv4(fd, &ip4_packet, dst));

    Ok(())
}

fn print_hex(vec: &Vec<u8>) {
    for byte in vec {
        print!("{:02x} ", byte);
    }
    println!();
}
