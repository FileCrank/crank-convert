/*
Utility functions for working with the DataHolder struct
 */

use std::io::Read;
use crate::{CrankResult, DataHolder};

#[inline]
pub fn read_holder_to_string(holder: &mut DataHolder) -> CrankResult<String> {
    let mut data_str: String = String::new();
    holder.read_to_string(&mut data_str)?;
    Ok(data_str)
}