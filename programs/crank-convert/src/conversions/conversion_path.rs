use crate::conversions::ConversionChain;
use crate::file_types::CrankFileType;
use crate::{ConversionNotFoundError, CrankResult};

pub fn find_conversion(
    from: &'static CrankFileType,
    to: &'static CrankFileType,
) -> CrankResult<&'static ConversionChain> {
    match from.conversions.get(to.name) {
        Some(chain) => Ok(chain),
        None => Err(ConversionNotFoundError(from, to)),
    }
}
