#[cfg(feature="native")]
use std::path::Path;

pub struct Opts<'a> {
    pub data: Option<Box<Vec<u8>>>,

    #[cfg(feature="native")]
    pub file: Option<&'a Path>,

    #[cfg(feature="native")]
    pub stream: bool
}

impl Default for Opts<'_> {
    #[cfg(feature="native")]
    fn default() -> Self {
        Self {
            data: None,
            file: None,
            stream: false,
        }
    }

    #[cfg(not(feature="native"))]
    fn default() -> Self {
        Self {
            data: None
        }
    }
}

impl<'a> Opts<'a> {
    #[cfg(feature="native")]
    pub fn from_file(path: &'a Path) -> Self {
        Self {
            data: None,
            file: Some(path),
            ..Self::default()
        }
    }

    pub fn from_data(data: Box<Vec<u8>>) -> Self {
        Self {
            data: Some(data),
            ..Self::default()
        }
    }
}