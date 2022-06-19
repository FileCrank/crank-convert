use crate::{CrankResult, DataHolder};

#[inline]
/// The identity conversion makes no changes to the data
pub fn identity_conversion(_: &mut DataHolder) -> CrankResult<()> {
    Ok(())
}
