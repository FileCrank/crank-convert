use crate::file_types::CrankFileType;
use thiserror::Error;
use serde_json::{Error as SerdeJSONError};
use csv::{Error as CSVError};

#[derive(Error, Debug)]
pub enum CrankError {
    #[error("No conversion found from {0} to {1}")]
    ConversionNotFoundError(&'static CrankFileType, &'static CrankFileType),

    // Various types of malformed data
    #[error("Malformed data")]
    MalformedJSONError(#[from] SerdeJSONError),
    #[error("Malformed data")]
    MalformedCSVError(#[from] CSVError),
}

pub type CrankResult<T> = Result<T, CrankError>;
