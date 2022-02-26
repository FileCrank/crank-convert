use std::fmt::Write;
use std::io::{BufReader, Read};

/// A simple struct that can hold data either from a file (if native),
/// or a


pub struct SimpleConvertable<T>
where T: Read+Write {
    pub data: BufReader<T>
}

impl<T> Read for SimpleConvertable<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.data.read(buf)
    }
}

impl<T> Write for SimpleConvertable<T> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.data.write_str(s)
    }
}

impl<T> SimpleConvertable<T> {
    pub fn new() {

    }
}