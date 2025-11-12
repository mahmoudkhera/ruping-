use std::net::{Ipv4Addr, UdpSocket};

use crate::{
    icmp::{self, RawICMP},
    ip_data::{IpProtocol, RawIpv4},
    socket::{recive, send_ipv4},
};

pub fn process_income_packet(fd:i32) {
    //suppse the mtu is 1500 and and may be another 100 addtiona bytes so buffer size is 1600
    let mut buf = [0u8; 1800];


    recive(fd, &mut buf[..]);

    //note that we do not need to process the incoming ip header
    //the kernal do it for us because the fd is icmp not raw

    let (_, ihl) = RawIpv4::read_ip_header(&buf);

    println!("data_offset {}", ihl);

    let icmp_header = RawICMP::from_buf(&mut buf[ihl..]);

    println!("kind {:?}", icmp_header.get_kind());
}

pub fn send_echo(dst: &str, fd: i32) {
    let data = b"this is the echo message why not working";

    //make an icmp echo message
    let mut icmp = RawICMP::new(icmp::Kind::Echo);

    let icmp = icmp.evaluate_icmp(data);

    // Set the parameters of ip header manually

    let src = get_local_ip(dst);

    // We do unwrap here because the get local check the correctness of the dst
    let dst = dst.parse().unwrap();
    let mut ipv4 = RawIpv4::new(0, 222, src, dst);

    let ip4_packet = ipv4.evaluate_ipv4(IpProtocol::ICMP, &icmp);

    println!("{:?}", send_ipv4(fd, &ip4_packet, dst));
}

fn get_local_ip(dst: &str) -> Ipv4Addr {
    let sock = UdpSocket::bind("0.0.0.0:0").unwrap();

    sock.connect((dst, 80)).unwrap();
    let local_addr = sock.local_addr().unwrap();

    if let std::net::SocketAddr::V4(ipv4) = local_addr {
        *ipv4.ip()
    } else {
        panic!("Destination is IPv6  or wrong , expected right  IPv4");
    }
}

fn print_hex(vec: &Vec<u8>) {
    for byte in vec {
        print!("{:02x} ", byte);
    }
    println!();
}
