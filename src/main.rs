use std::net::Ipv4Addr;

use ruping::{
    ip_data::{IpProtocol, RawIpv4},
    socket::{create_sock, set_socket},
};

fn main() {
    let fd = create_sock().unwrap();

    println!("test socket creation {:?}", set_socket(fd).is_ok());
}
