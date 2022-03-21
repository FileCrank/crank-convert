#[cfg(feature = "native")]
use log::LevelFilter;
#[cfg(feature = "native")]
use std::path::Path;
use crate::file_types::document::text::TXT;
use crate::file_types::file_type::FileType;

#[derive(Clone, Debug)]
pub enum OptFileType<'a> {
    Name(String),
    Type(&'a FileType<'static>)
}

#[derive(Debug, Clone)]
pub struct Opts<'a> {
    pub data: Option<&'a Box<Vec<u8>>>,

    pub from_file_type: Option<OptFileType<'a>>,
    pub to_file_type: OptFileType<'a>,

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
            to_file_type: OptFileType::Name("".to_string()),
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
            to_file_type: OptFileType::Name("".to_string())
        }
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
            data: Some(&data),
            ..Self::default()
        }
    }
}
