use crate::formats::data_format::DataFormat;
use crate::errors::{ConversionError, Result};

/// Simply read the data from `from` to `to`
pub fn simple_copy<F, T>(from: &mut Box<F>,
                   to: &mut Box<T>) -> Result<usize>
where F: DataFormat, T: DataFormat {
    match from.read(to) {
        Ok(u) => Ok(u),
        Err(e) => ConversionError::IOError(e)
    }
}