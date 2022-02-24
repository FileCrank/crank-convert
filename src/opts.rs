#[cfg(feature="std")]
use std::path::Path;

pub struct Opts<'a> {
    pub data: Option<Box<Vec<u8>>>,

    #[cfg(feature="std")]
    pub file: Option<&'a Path>,

    #[cfg(feature="std")]
    pub stream: bool
}

impl Default for Opts {
    #[cfg(feature="std")]
    fn default() -> Self {
        Self {
            data: None,
            file: None,
            stream: false,
        }
    }

    #[cfg(not(feature="std"))]
    fn default() -> Self {
        Self {
            data: None
        }
    }
}

impl Opts {
    #[cfg(feature="std")]
    pub fn from_file(path: &Path) -> Self {
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