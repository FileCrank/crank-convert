use std::io::{Read, Write};
use serde_json::Value;
use crate::data_formats::json::JSONFormat;

pub enum StructuredDataFormat {
    JSON(JSONFormat)
}

impl Read for StructuredDataFormat {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            StructuredDataFormat::JSON(j) => j.read(buf)
        }
    }
}

impl Write for StructuredDataFormat {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            StructuredDataFormat::JSON(j) => j.write(buf)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            StructuredDataFormat::JSON(j) => j.flush()
        }
    }
}