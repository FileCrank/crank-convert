use crate::file_types::file_type::FileType;
use lazy_static::lazy_static;
#[cfg(feature = "native")]
use log::LevelFilter;
use std::ops::Deref;
#[cfg(feature = "native")]
use std::path::Path;

#[derive(Clone, Debug)]
pub enum OptFileType<'a> {
    Name(String),
    Type(&'a FileType<'static>),
}
lazy_static! {
    pub static ref DEFAULT_OFT: OptFileType<'static> = OptFileType::Name("".to_string());
}

#[derive(Debug, Clone)]
pub struct Opts<'a> {
    pub data: Option<&'a Box<Vec<u8>>>,

    pub from_file_type: Option<&'a OptFileType<'a>>,
    pub to_file_type: &'a OptFileType<'a>,

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
            from_file_type: None,
            to_file_type: &DEFAULT_OFT.deref(),
            file: None,
            stream: false,
            log_level: LevelFilter::Info,
        }
    }

    #[cfg(not(feature = "native"))]
    fn default() -> Self {
        Self {
            data: None,
            from_file_type: None,
            to_file_type: &DEFAULT_OFT.deref(),
            to_file_type: OptFileType::Name("".to_string()),
        }
    }
}

impl<'a> Opts<'a> {
    #[cfg(feature = "native")]
    pub fn from_file(path: &'a Path, to_file_type: &'a OptFileType) -> Self {
        Self {
            data: None,
            file: Some(path),
            to_file_type,
            ..Self::default()
        }
    }

    pub fn from_data(data: &'a Box<Vec<u8>>, to_file_type: &'a OptFileType) -> Self {
        Self {
            data: Some(data),
            to_file_type,
            ..Self::default()
        }
    }
}
