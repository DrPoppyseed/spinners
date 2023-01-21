# Spinners - 🛎 60+ Elegant terminal spinners for Rust

[![Cargo version](https://img.shields.io/crates/v/spinners.svg)](https://crates.io/crates/spinners) [![Crates.io](https://img.shields.io/crates/l/spinners.svg)](https://crates.io/crates/spinners) [![docs.rs](https://img.shields.io/badge/docs.rs-👌-4EC329.svg?)](https://docs.rs/spinners/) [![Crates.io](https://img.shields.io/crates/d/spinners.svg)](https://crates.io/crates/spinners) [![Slack](https://img.shields.io/badge/Slack-Join%20our%20tech%20community-17202A?logo=slack)](https://join.slack.com/t/fgribreau/shared_invite/zt-edpjwt2t-Zh39mDUMNQ0QOr9qOj~jrg)

<p align="center"><img src="https://media.giphy.com/media/3oxHQyZfOJjlL3bhRK/giphy.gif"></p>

> ## ❤️ fgribreau's other amazing creations
>
> - [**Charts, simple as a URL**. No more server-side rendering pain, 1 url = 1 chart](https://image-charts.com)
> - [Keycloak Identity and Access Management (IAM) as a Service](https://www.cloud-iam.com/)
> - [Automate your RoamResearch second brain](https://www.roam-bot.com)

## Install

See [Cargo page](https://crates.io/crates/spinners)

## Usage

```rust
use atomic_spinners::SpinnerBuilder;
use std::{thread::sleep, time::Duration};

fn main() {
    let mut sp = Spinner::new()
        .message("Waiting for 3 seconds")
        .build();
    sleep(Duration::from_secs(3));
    sp.stop();
}
```

## Example

```shell
cargo run --example simple
```

## License

MIT © [François-Guillaume Ribreau](https://fgribreau.com), James Cordor, 
[Haruki Jay Shimada](https://github.com/DrPoppyseed)
