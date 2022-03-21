use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Got IO Error while trying to perform copy")]
    IOError(#[from] std::io::Error),

    #[error("Unsupported conversion")]
    UnsupportedConversionError,

    #[error("Invalid configuration: {0}")]
    InvalidConfigurationError(&'static str),

    #[error("File type {0} not found")]
    FileTypeNotFoundError(String),

    #[error("File type of source file not provided")]
    FileTypeNotProvidedError
}

pub type Result<T> = std::result::Result<T, ConversionError>;
