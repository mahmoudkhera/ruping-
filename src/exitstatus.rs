use std::{
    ffi::c_int,
    sync::atomic::{AtomicBool, Ordering},
};

pub static SIGANAL_RECIVED: AtomicBool = AtomicBool::new(false);

// FFI for signal handling (add to your existing FFI section)
const SIGINT: c_int = 2;

unsafe extern "C" {
    fn signal(signum: c_int, handler: usize) -> usize;
}

extern "C" fn sigint_handler(_: c_int) {
    SIGANAL_RECIVED.store(true, Ordering::SeqCst);
}

pub fn setup_sigint_handler() {
    unsafe {
        signal(SIGINT, sigint_handler as usize);
    }
}

pub struct ExistStatus {
    transmitted: u32,
    received: u32,
    total_rtt: f64,
}

impl ExistStatus {
    pub fn new() -> Self {
        Self {
            transmitted: 0,
            received: 0,
            total_rtt: 0.0,
        }
    }
    pub fn print_summary(&self) {
        let packet_loss = (self.transmitted - self.received) / self.transmitted * 100;

        let avarage_rtt = self.total_rtt / self.received as f64;
        println!(
            "\n--- ping statistics ---\n\
             {} packets transmitted, {} received, {:.0}% packet loss, avarage rtt {:.2}ms\n\
             ",
            self.transmitted, self.received, packet_loss, avarage_rtt,
        );
    }

    pub fn add_trasn(&mut self) {
        self.transmitted += 1;
    }

    pub fn add_recv(&mut self) {
        self.received += 1;
    }
    pub fn add_rtt(&mut self, rtt: f64) {
        self.total_rtt += rtt;
    }
}
