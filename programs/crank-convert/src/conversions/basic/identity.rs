use crate::file_data::FileData;

/// The identity conversion just returns the data that was given to it
pub fn identity(data: &mut FileData) -> &mut FileData {
    data
}
