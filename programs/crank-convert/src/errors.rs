use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConversionError>;

#[derive(Error, Debug)]
pub enum ConversionError {

}