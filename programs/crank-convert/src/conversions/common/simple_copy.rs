use crate::errors::{ConversionError, Result};
use crate::formats::data_format::DataFormat;

/// Simply transfer the data from the source
pub fn simple_copy(from: &mut Box<dyn DataFormat>,
                   to: &mut Box<dyn DataFormat>) -> Result<usize> {
    match to.consume_buf(from) {
        Ok(u) => Ok(u),
        Err(e) => Err(ConversionError::IOError(e))
    }
}