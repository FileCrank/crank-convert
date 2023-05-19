use crate::file_types::CrankFileType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrankError {
    #[error("No conversion found from {0} to {1}")]
    ConversionNotFoundError(&'static CrankFileType, &'static CrankFileType),
}

pub type CrankResult<T> = Result<T, CrankError>;
