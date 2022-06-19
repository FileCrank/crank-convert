use crate::conversion::{execute_chain, ConversionChain};
use crate::convertable::CONVERTABLE;
use crate::errors::ConversionError::ConversionNotFoundError;
use crate::errors::CrankResult;
use crate::file_types::crank_file_type::CrankFileType;
use std::fs::FileType;
use crate::file_data::FileData;
use crate::file_data::DataHolder;

pub mod conversion;
pub mod conversions;
pub mod convertable;
pub mod errors;
pub mod file_data;
pub mod file_types;


pub fn find_conversion(
    from: &'static CrankFileType,
    to: &'static CrankFileType,
) -> Option<&'static ConversionChain> {
    // Search through the available conversions
    from.conversions.get(to.name)
}

pub fn convert(data: &mut FileData, to: &'static CrankFileType) -> CrankResult<()> {
    let conv = find_conversion(data.file_type, to);
    match conv {
        Some(chain) => {
            execute_chain(data, chain)?;
            data.file_type = to;
            Ok(())
        },
        None => Err(ConversionNotFoundError(data.file_type, to)),
    }
}
