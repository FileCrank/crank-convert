use std::io::{Read, Write};
use std::ops::Deref;
use std::convert::From;
use crate::errors::Result;

pub trait DataFormat<'a>: Read + Write + From<&'a mut dyn DataFormat<'a>> {
    fn consume_buf(&mut self, buf: &mut Box<dyn DataFormat>) -> std::io::Result<usize>;
}
