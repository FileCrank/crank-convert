use crate::CrankFileType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("No conversion found from {0} to {1}")]
    ConversionNotFoundError(&'static CrankFileType, &'static CrankFileType),
}

pub type CrankResult<T> = Result<T, ConversionError>;
