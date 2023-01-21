#[cfg(feature = "dots9")]
use atomic_spinners::{variants, Spinner};

#[cfg(feature = "dots9")]
use std::{thread::sleep, time::Duration};

#[cfg(feature = "dots9")]
fn runner() {
    let mut sp = Spinner::new(variants::Dots9, "Waiting for 3 seconds");
    sleep(Duration::from_secs(3));
    sp.stop_and_persist("✔", "That worked!".to_string())
}

fn main() {
    #[cfg(feature = "dots9")]
    runner();
}
