use std::io::{BufReader, Read};
use crate::errors::Result;
use crate::conversions::convertable::Convertable;
use crate::errors::ConversionError::IOError;

/// A function that can be used for any conversions that
/// are basically just "Copy the data"
pub fn simple_copy<S, D>(source: S, dest: D) -> Result<usize>
where S: Convertable, D: Convertable {
    let mut source_buf = source.get_buf();
    let mut dest_buf = dest.get_buf();
    match source_buf.read(dest_buf) {
        Ok(u) => Ok(u),
        Err(e) => IOError(e)
    }
}
