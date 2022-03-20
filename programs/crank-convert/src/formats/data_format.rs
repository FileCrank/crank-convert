use crate::formats::data_holder::DataHolder;
use crate::utils::ReadWrite;
use std::io::{Cursor, Read, Write};

pub enum DataFormat {
    Bytes(Cursor<Vec<u8>>),
}

pub fn initialize_bytes(holder: &mut DataHolder) -> DataFormat {
    let mut data = Vec::new();
    holder.read_to_end(&mut data).unwrap();
    DataFormat::Bytes(Cursor::new(data))
}

pub fn empty_bytes() -> DataFormat {
    DataFormat::Bytes(Cursor::new(Vec::new()))
}

/// Used as a shorthand for doing something on all variants
macro_rules! all_formats {
    ($self: ident, $fn: ident) => {
        match $self {
            DataFormat::Bytes(v) => v.$fn(),
        }
    };
    ($self: ident, $fn: ident, $with: expr) => {
        match $self {
            DataFormat::Bytes(v) => v.$fn($with),
        }
    };
}

impl ReadWrite for DataFormat {}

impl Read for DataFormat {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        all_formats!(self, read, buf)
    }
}

impl Write for DataFormat {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        all_formats!(self, write, buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        all_formats!(self, flush)
    }
}
