use std::thread;

use ruping::app::{process_income_packet, send_echo};
use ruping::socket;
use ruping::socket::create_raw_sock;

fn main() {
    let send_fd = create_raw_sock(socket::IPPROTO_RAW).unwrap();
    let recive_fd = create_raw_sock(socket::IPPROTO_ICMP).unwrap();

    thread::spawn(move || {
        loop {
            process_income_packet(recive_fd);
        }
    });

    let dst = "192.168.13.6";

    for _ in 0..=3 {
        send_echo(dst, send_fd);
    }
}
