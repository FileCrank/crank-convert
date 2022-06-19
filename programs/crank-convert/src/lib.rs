use crate::data_holder::DataHolder;
use crate::opts::CrankOpts;

pub mod errors;
pub mod data_holder;
pub mod opts;
pub mod conversions;
pub mod file_types;
pub mod macros;

pub use errors::*;

pub fn convert(data: &mut DataHolder,
               opts: CrankOpts) {

}