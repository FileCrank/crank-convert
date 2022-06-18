use crate::file_types::crank_file_type::CrankFileType;
use crate::file_types::document::rtf::RTF;
use crate::file_types::document::txt::TXT;
use lazy_static::lazy_static;
use std::cell::Ref;
use std::fs::FileType;

lazy_static! {
    pub static ref CONVERTABLE: Vec<&'static CrankFileType> = vec![&TXT, &RTF];
}
