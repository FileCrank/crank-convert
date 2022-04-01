use std::io::{Cursor, Read, Write};
use crate::data_formats::{ArchiveDataFormat, ExecutableDataFormat, StructuredDataFormat};

pub enum DataFormat {
    Raw(Cursor<Vec<u8>>),
    Structured(StructuredDataFormat),
    Archive(ArchiveDataFormat),
    Executable(ExecutableDataFormat)
}

/// Used as a shorthand for doing something on all variants. The conversion set is that all
/// top level formats should be Read + Write, with the actual details of that handled below
macro_rules! all_formats {
    ($self: ident, $fn: ident) => {
        match $self {
            DataFormat::Raw(v) => v.$fn(),
            DataFormat::Structured(s) => s.$fn(),
            DataFormat::Archive(a) => a.$fn(),
            DataFormat::Executable(e) => e.$fn()
        }
    };
    ($self: ident, $fn: ident, $with: expr) => {
        match $self {
            DataFormat::Raw(v) => v.$fn($with),
            DataFormat::Structured(s) => s.$fn($with),
            DataFormat::Archive(a) => a.$fn($with),
            DataFormat::Executable(e) => e.$fn($with)
        }
    };
}

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

pub struct FormatTag {
    name: &'static str,
    initializer: fn(DataFormat) -> DataFormat,
    empty: fn() -> DataFormat
}