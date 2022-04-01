use std::io::{Read, Write};

pub enum ExecutableDataFormat {

}

impl Read for ExecutableDataFormat {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unreachable!()
    }
}

impl Write for ExecutableDataFormat {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unreachable!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        unreachable!()
    }
}