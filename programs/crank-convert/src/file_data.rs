use crate::CrankFileType;
use std::fs::File;
use std::io::BufReader;

pub struct FileData {
    pub data: BufReader<File>,
    pub file_type: &'static CrankFileType,
}
