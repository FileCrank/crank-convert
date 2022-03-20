use crate::opts::Opts;
#[cfg(feature = "native")]
use simple_logger::SimpleLogger;

pub mod constants;
pub mod conversions;
pub mod errors;
pub mod file_types;
pub mod formats;
pub mod interface;
pub mod macros;
pub mod opts;
pub mod utils;

pub use macros::*;

pub fn main(opts: Opts) {
    // Initialize logging if we're running natively. If we're not,
    // and we don't initialize a backend, the log macros will just silently
    // do nothing
    #[cfg(feature = "native")]
    SimpleLogger::new()
        .with_level(opts.log_level)
        .init()
        .unwrap();

    log::debug!("Starting file conversion with opts {:?}", opts);
}
