use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Got IO Error while trying to perform copy")]
    IOError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ConversionError>;
