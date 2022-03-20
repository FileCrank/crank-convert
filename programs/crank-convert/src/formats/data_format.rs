use std::io::{Cursor, Read, Write};
use std::ops::Deref;
use std::convert::From;
use crate::errors::Result;
use crate::formats::data_holder::DataHolder;
use crate::utils::ReadWrite;

pub enum DataFormat {
    Bytes(Cursor<Vec<u8>>)
}

pub fn initialize_bytes(holder: &mut DataHolder) -> DataFormat {
    let mut data = Vec::new();
    holder.read_to_end(&mut data);
    DataFormat::Bytes(Cursor::new(data))
}

pub fn empty_bytes() -> DataFormat {
    DataFormat::Bytes(Cursor::new(Vec::new()))
}

/// Used as a shorthand for doing something on all variants
macro_rules! all_formats {
    ($self: ident, $action: expr) => {
        match $self {
            DataFormat::Bytes(v) => $action(&v)
        }
    }
}

impl ReadWrite for DataFormat {}

impl Read for DataFormat {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        all_formats!(self, |x: &dyn Read| {
            x.read(buf)
        })
    }
}

impl Write for DataFormat {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        all_formats!(self, |x: &dyn Write | {
            x.write(buf)
        })
    }

    fn flush(&mut self) -> std::io::Result<()> {
        all_formats!(self, |x: &dyn Write | {
            x.flush()
        })
    }
}