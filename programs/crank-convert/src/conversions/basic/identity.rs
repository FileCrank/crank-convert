use crate::{DataHolder};
use anyhow::Result;

#[inline]
/// The identity conversion makes no changes to the data
pub fn identity_conversion(holder: &mut DataHolder) -> Result<()> {
    Ok(())
}
