use std::any::Any;
#[cfg(feature="native")]
use std::fs::File;
use std::io::{Read, Write};
use crate::utils::ReadWrite;

pub enum DataHolder {
    Raw(Box<Vec<u8>>),
    #[cfg(feature="native")]
    File(Box<File>)
}


fn read_raw(data: &mut Vec<u8>, buf: &mut [u8]) -> std::io::Result<usize> {
    data.as_slice().read(buf)
}

#[cfg(feature="native")]
fn read_file(file: &mut File, buf: &mut [u8]) -> std::io::Result<usize> {
    file.read(buf)
}


impl Read for DataHolder {

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if cfg!(feature="native") {
            match self {
                DataHolder::Raw(r) => read_raw(r, buf),
                DataHolder::File(f) => read_file(f, buf)
            }
        } else {
            match self {
                DataHolder::Raw(r) => read_raw(r, buf),
                _ => {
                    unreachable!("Only raw data should be accessible in non-native");
                }
            }
        }
    }
}

impl Write for DataHolder {

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
       match self {
           DataHolder::Raw(r) => r.write(buf),
           DataHolder::File(f) => f.write(buf),
       }
    }


    fn flush(&mut self) -> std::io::Result<()> {
       match self {
           DataHolder::Raw(r) => r.flush(),
           DataHolder::File(f) => f.flush()
       }
    }
}

impl Default for DataHolder {
    fn default() -> Self {
        Self::Raw(Box::new(Vec::new()))
    }
}