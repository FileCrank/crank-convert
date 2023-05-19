use std::fs::File;
use std::io::{Read, Write};

/// The data holder enum is how data will be wokred with throughout the system
pub enum DataHolder {
    File(File),
    Bytes(Vec<u8>),
}

impl Read for DataHolder {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            DataHolder::File(f) => f.read(buf),
            DataHolder::Bytes(b) => b.as_slice().read(buf)
        }
    }
}

impl Write for DataHolder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            DataHolder::File(f) => f.write(buf),
            DataHolder::Bytes(b) => b.write(buf)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            DataHolder::File(f) => f.flush(),
            DataHolder::Bytes(b) => b.flush()
        }
    }
}
