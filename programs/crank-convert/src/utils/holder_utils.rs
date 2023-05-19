/*
Utility functions for working with the DataHolder struct
 */
use crate::{DataHolder};
use anyhow::Result;
use std::io::Read;

#[inline]
pub fn read_holder_to_string(holder: &mut DataHolder) -> Result<String> {
    let mut data_str: String = String::new();
    holder.read_to_string(&mut data_str)?;
    Ok(data_str)
}
