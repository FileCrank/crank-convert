#[cfg(feature = "native")]
use log::LevelFilter;
#[cfg(feature = "native")]
use std::path::Path;

#[derive(Debug)]
pub struct Opts<'a> {
    pub data: Option<Box<Vec<u8>>>,

    #[cfg(feature = "native")]
    pub file: Option<&'a Path>,

    #[cfg(feature = "native")]
    pub stream: bool,

    #[cfg(feature = "native")]
    pub log_level: LevelFilter,
}

impl Default for Opts<'_> {
    #[cfg(feature = "native")]
    fn default() -> Self {
        Self {
            data: None,
            file: None,
            stream: false,
            log_level: LevelFilter::Info,
        }
    }

    #[cfg(not(feature = "native"))]
    fn default() -> Self {
        Self { data: None }
    }
}

impl<'a> Opts<'a> {
    #[cfg(feature = "native")]
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
