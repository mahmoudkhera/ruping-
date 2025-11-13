use ruping::{app::app_start, exitstatus};

fn main() {
    exitstatus::setup_sigint_handler();

    let dst = "8.8.8.8";

    app_start(dst);
}
