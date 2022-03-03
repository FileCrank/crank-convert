use std::io::{Read, Write};
use std::ops::Deref;
use crate::errors::Result;

pub trait DataFormat: Read + Write {
    fn consume_buf(&mut self, buf: &mut Box<dyn DataFormat>) -> std::io::Result<(usize)>;
}
