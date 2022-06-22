use crate::data_holder::DataHolder;
use crate::opts::CrankOpts;

pub mod conversions;
pub mod data_holder;
pub mod errors;
pub mod file_types;
pub mod macros;
pub mod opts;

mod utils;
use utils::*;

use crate::conversions::conversion_path::find_conversion;
use crate::conversions::ConversionChain;
use crate::CrankError::ConversionNotFoundError;
pub use errors::*;

/// Go through all the conversions in a chain, and execute them one by one
pub fn execute_conversion(data: &mut DataHolder, chain: &ConversionChain) -> CrankResult<()> {
    for conv in chain {
        (conv.conversion)(data)?;
    }
    Ok(())
}

pub fn convert(data: &mut DataHolder, opts: CrankOpts) -> CrankResult<()> {
    let chain = find_conversion(opts.from, opts.to)?;
    execute_conversion(data, chain)
}
