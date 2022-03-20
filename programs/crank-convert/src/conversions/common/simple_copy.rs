use std::io::{Read, Write};
use crate::errors::{ConversionError, Result};
use crate::errors::ConversionError::IOError;
use crate::formats::data_format::DataFormat;

/// Simply transfer the data from the source
pub fn simple_copy(from: &mut Box<DataFormat>,
                   to: &mut Box<DataFormat>) -> Result<usize> {
    let mut buf: Vec<u8> = Vec::new();
    from.read(buf.as_mut_slice());
    match to.write(buf.as_mut_slice()) {
        Ok(u) => Ok(u),
        Err(e) => Err(IOError(e))
    }
}