use atomic_spinners::SpinnerBuilder;
use std::{thread::sleep, time::Duration};

fn main() {
    let mut sp = SpinnerBuilder::new()
        .message("Waiting for 3 seconds")
        .timer()
        .build();
    sleep(Duration::from_secs(3));
    sp.stop_with_newline();
}
