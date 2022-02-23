use spinners_rs::{Spinner, Spinners};
use std::{env, str::FromStr, thread::sleep, time::Duration};

fn main() {
    let mut args = env::args();
    let spinner_name = args.nth(1).unwrap_or_else(|| "Dots9".to_string());

    let sp = Spinner::new(
        Spinners::from_str(&spinner_name).unwrap(),
        "Waiting for 3 seconds".into(),
    );
    sleep(Duration::from_secs(3));
    sp.stop_with_message("Finishing waiting for 3 seconds\n".into());
}
