use std::io::{Read, Write};
use crate::formats::data_format::DataFormat;
use crate::formats::data_holder::DataHolder;
use crate::utils::ReadWrite;

/// Format that just holds the data, and derefs read and write
pub struct SimpleDataFormat {
    data: DataHolder
}

impl Read for SimpleDataFormat {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.data.read(buf)
    }
}

impl Write for SimpleDataFormat {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.data.flush()
    }
}

impl Default for SimpleDataFormat {
    fn default() -> Self {
        Self {
            data: DataHolder::default()
        }
    }
}

impl ReadWrite for SimpleDataFormat {}

impl DataFormat for SimpleDataFormat {
    fn new(data: DataHolder) -> Self {
        Self {
            data
        }
    }
}

