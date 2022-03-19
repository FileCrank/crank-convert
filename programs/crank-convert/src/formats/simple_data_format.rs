use std::io::{Read, Write};
use crate::formats::data_format::DataFormat;
use crate::formats::data_holder::DataHolder;

pub struct SimpleDataFormat {
    inner: Vec<u8>
}

impl Read for SimpleDataFormat {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.as_slice().read(buf)
    }
}

impl Write for SimpleDataFormat {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl From<&'_ mut dyn DataFormat<'_>> for SimpleDataFormat {
    fn from(source: &'_ mut dyn DataFormat) -> Self {
        let mut inner = Vec::new();
        source.read(inner.as_mut_slice());
        Self {
            inner
        }
    }
}

impl DataFormat<'_> for SimpleDataFormat {
    fn consume_buf(&mut self, buf: &mut Box<dyn DataFormat>) -> std::io::Result<usize> {
        buf.read_to_end(&mut self.inner)
    }
}

impl SimpleDataFormat {

    pub fn new() -> Self {
        Self {
            inner: Vec::new()
        }
    }

    pub fn from_holder(holder: &mut DataHolder) -> Self {
        let mut data = Vec::new();
        holder.read_to_end(&mut data);
        Self {
            inner: data
        }
    }
}