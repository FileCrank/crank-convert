use crate::CrankFileType;
use std::fs::File;
use std::io::{BufReader, Read};

pub enum DataHolder<'a> {
    File(File),
    Bytes(&'a [u8])
}

impl Read for DataHolder<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            DataHolder::File(f) => f.read(buf),
            DataHolder::Bytes(b) => b.read(buf)
        }
    }
}

pub struct FileData<'a> {
    pub data: BufReader<DataHolder<'a>>,
    pub file_type: &'static CrankFileType,
}
