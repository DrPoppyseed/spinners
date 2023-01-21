#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
mod spinner;
pub use spinner::{Spinner, SpinnerBuilder};

mod stream;
pub use stream::Stream;

pub mod variants;

pub use once_cell;