use crate::conversion::ConversionChain;
use crate::convertable::CONVERTABLE;
use crate::errors::ConversionError::ConversionNotFoundError;
use crate::errors::CrankResult;
pub use crate::file_data::FileData;
use crate::file_types::crank_file_type::CrankFileType;
use std::fs::FileType;

pub mod conversion;
mod conversions;
pub mod convertable;
pub mod errors;
mod file_data;
pub mod file_types;

pub fn find_conversion(
    from: &'static CrankFileType,
    to: &'static CrankFileType,
) -> Option<&'static ConversionChain> {
    // Search through the available conversions
    from.conversions.get(to.name)
}

pub fn convert<'a>(
    data: &'a mut FileData,
    to: &'static CrankFileType,
) -> CrankResult<&'a mut FileData> {
    let conv = find_conversion(data.file_type, to);
    match conv {
        Some(chain) => Ok(data),
        None => Err(ConversionNotFoundError(data.file_type, to)),
    }
}
