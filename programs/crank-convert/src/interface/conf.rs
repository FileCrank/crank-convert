use crate::errors::ConversionError::{FileTypeNotFoundError, FileTypeNotProvidedError, IOError};
use crate::errors::{ConversionError::InvalidConfigurationError, Result};
use crate::file_types::file_type::FileType;
use crate::formats::data_holder::DataHolder;
use crate::interface::type_names::{FILE_TYPE_EXTENSIONS, FILE_TYPE_NAMES};
use crate::opts::OptFileType;
use crate::Opts;
use std::ffi::OsString;
use std::fs::File;
use std::ops::Deref;
use std::path::Path;

#[cfg(feature = "native")]
fn read_file_to_holder(path: &Path) -> Result<DataHolder> {
    match File::open(path) {
        Ok(f) => Ok(DataHolder::File(Box::new(f))),
        Err(e) => Err(IOError(e)),
    }
}

#[cfg(feature = "native")]
fn parse_opts_holder(opts: &Opts) -> Result<DataHolder> {
    match (opts.data, opts.file) {
        (Some(d), None) => Ok(DataHolder::Raw(d.to_owned())),
        (None, Some(f)) => read_file_to_holder(f),
        (Some(_), Some(_)) => Err(InvalidConfigurationError(
            "Only data or file should be provided, not both",
        )),
        (None, None) => Err(InvalidConfigurationError("Data or file must be provided")),
    }
}

#[cfg(not(feature = "native"))]
fn parse_opts_holder(opts: &Opts) -> Result<DataHolder> {
    if let Some(data) = opts.data {
        Ok(DataHolder::Raw(data))
    } else {
        Err(InvalidConfigurationError("No data provided"))
    }
}

pub fn parse_opt_file_type<'a>(oft: &'a OptFileType<'a>) -> Result<&'a FileType<'static>> {
    match oft {
        OptFileType::Name(s) => {
            if let Some(ft) = FILE_TYPE_NAMES.get(&s.as_str()) {
                Ok(ft)
            } else {
                Err(FileTypeNotFoundError(s.to_string()))
            }
        }
        OptFileType::Type(f) => Ok(f),
    }
}

#[cfg(feature = "native")]
fn parse_extension_if_file(opts: &Opts) -> Result<OsString> {
    if let Some(path) = opts.file {
        if let Some(ext) = path.extension() {
            return Ok(ext.to_os_string().to_ascii_lowercase());
        }
    }
    Err(FileTypeNotProvidedError)
}

#[cfg(not(feature = "native"))]
fn parse_extension_if_file() -> Result<&'static str> {
    Err(FileTypeNotProvidedError)
}

fn parse_from_file_type<'a>(opts: &Opts<'a>) -> Result<&'a FileType<'static>> {
    match opts.from_file_type {
        Some(ft) => parse_opt_file_type(ft),
        None => {
            let extension_raw = parse_extension_if_file(opts)?;
            let extension = extension_raw.to_str().unwrap();
            if let Some(ft) = FILE_TYPE_EXTENSIONS.get(extension) {
                Ok(ft.deref())
            } else {
                Err(FileTypeNotProvidedError)
            }
        }
    }
}

pub struct ConversionConf<'a> {
    pub from: &'a FileType<'static>,
    pub to: &'a FileType<'static>,
    pub holder: DataHolder,
}

pub fn parse_validate_opts(opts: Opts) -> Result<ConversionConf> {
    match parse_opts_holder(&opts) {
        Ok(holder) => Ok(ConversionConf {
            from: parse_from_file_type(&opts)?,
            to: parse_opt_file_type(opts.to_file_type)?,
            holder,
        }),
        Err(e) => Err(e),
    }
}
