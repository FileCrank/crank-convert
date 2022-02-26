#[cfg(feature="native")]
use simple_logger::SimpleLogger;
use crate::opts::Opts;

pub mod conversions;
pub mod opts;
pub mod interface;
pub mod constants;
pub mod errors;
pub mod utils;
pub mod formats;

pub fn main(opts: Opts) {
    // Initialize logging if we're running natively. If we're not,
    // and we don't initialize a backend, the log macros will just silently
    // do nothing
    #[cfg(feature="native")]
    SimpleLogger::new().with_level(opts.log_level).init().unwrap();

    log::debug!("Starting file conversion with opts {:?}", opts);

}
