use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use ruping::app::{function_timeout, process_income_packet, send_echo};
use ruping::socket::{self, set_socket};
use ruping::socket::{create_raw_sock, recive};

fn main() {
    let send_fd = create_raw_sock(socket::IPPROTO_RAW).unwrap();

    set_socket(send_fd);
    let recive_fd = create_raw_sock(socket::IPPROTO_ICMP).unwrap();
    let buf = Arc::new(Mutex::new([0u8; 1024]));

    //that ping an ip in local LAN will force the kernal to add an ip header 
    // to your packet 
    let dst = "1.1.1.1";

    for _ in 0..4 {
        let start = Instant::now();
        send_echo(dst, send_fd);
        let buf_clone = Arc::clone(&buf); // clone the Arc

        let n = function_timeout(
            move || {
                let mut buf = buf_clone.lock().unwrap();
                recive(recive_fd, &mut *buf)
            },
            Duration::from_secs(1),
        );

        let rtt_ms = start.elapsed().as_secs_f64() * 1000.0;

        if n != 0 {
            let mut buf = buf.lock().unwrap();
            process_income_packet(&mut buf[..n], rtt_ms);
        } else {
            println!("Request timed out.");
        }
    }
}
