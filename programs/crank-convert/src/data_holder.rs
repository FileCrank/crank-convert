use std::fs::File;
use std::io::{Read, Write};

/// The data holder enum is how data will be wokred with throughout the system
pub enum DataHolder<'a> {
    File(File),
    Bytes(&'a mut [u8]),
}

impl Read for DataHolder<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            DataHolder::File(f) => f.read(buf),
            DataHolder::Bytes(b) => {
                // I'm not sure why I have to write this like this, but I get compiler errors
                // if I don't explicitly separate this. Hopefully this line doesn't cause an
                // unexpected copy
                let mut sl: &[u8] = *b;
                sl.read(buf)
            }
        }
    }
}

impl Write for DataHolder<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            DataHolder::File(f) => f.write(buf),
            DataHolder::Bytes(b) => b.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            DataHolder::File(f) => f.flush(),
            DataHolder::Bytes(b) => b.flush(),
        }
    }
}
