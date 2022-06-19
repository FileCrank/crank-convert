use crate::file_data::FileData;
use crate::CrankResult;

/// The identity conversion just returns the data that was given to it
pub fn identity(_: &mut FileData) -> CrankResult<()> {
    Ok(())
}
