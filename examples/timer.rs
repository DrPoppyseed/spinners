#[cfg(feature = "dots")]
use atomic_spinners::{variants, Spinner};

#[cfg(feature = "dots")]
use std::{thread::sleep, time::Duration};

#[cfg(feature = "dots")]
fn runner() {
    let mut sp = Spinner::with_timer(variants::Dots, "Waiting for 3 seconds");
    sleep(Duration::from_secs(3));
    sp.stop_with_newline();
}

fn main() {
    #[cfg(feature = "dots")]
    runner()
}
