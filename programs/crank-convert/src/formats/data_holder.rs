#[cfg(feature="native")]
use std::fs::File;
use std::io::{Read, Write};
use crate::utils::ReadWrite;

pub enum DataHolder {
    Raw(Box<Vec<u8>>),
    #[cfg(feature="native")]
    File(Box<File>)
}

impl DataHolder {

    #[cfg(feature="native")]
    #[inline]
    fn get_inner(&mut self) -> &mut Box<dyn ReadWrite>{
        match self {
            DataHolder::Raw(r) => r,
            DataHolder::File(f) => f
        }
    }

    #[cfg(not(feature="native"))]
    #[inline]
    fn get_inner(&mut self) -> &mut Box<dyn ReadWrite> {
        match self {
            DataHolder::Raw(r) => r,
            _ => {
                unreachable!("Only raw data should be accessible in non-native");
            }
        }
    }
}

impl Read for DataHolder {

    fn read(& mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.get_inner().read(buf)
    }
}

impl Write for DataHolder {

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.get_inner().write(buf)
    }


    fn flush(&mut self) -> std::io::Result<()> {
       self.get_inner().flush()
    }
}

impl Default for DataHolder {
    fn default() -> Self {
        Self::Raw(Box::new(Vec::new()))
    }
}