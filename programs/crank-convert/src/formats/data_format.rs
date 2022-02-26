use std::io::{BufReader, BufWriter};
use crate::formats::data_holder::DataHolder;
use crate::utils::ReadWrite;

pub trait DataFormat: ReadWrite {
    fn new(data: DataHolder);

    // Don't implement Default because of size, but require the same behavior
    fn default() -> Self;
}