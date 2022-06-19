use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrankError {

}

pub type CrankResult<T> = Result<T, CrankError>;