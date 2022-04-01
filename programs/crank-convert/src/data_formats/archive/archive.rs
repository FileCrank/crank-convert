use std::io::{Read, Write};

pub enum ArchiveDataFormat {

}

impl Read for ArchiveDataFormat {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unreachable!()
    }
}

impl Write for ArchiveDataFormat {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unreachable!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        unreachable!()
    }
}