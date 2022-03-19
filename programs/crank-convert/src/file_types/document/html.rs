use lazy_static::lazy_static;
use crate::{converts_to, set};
use crate::file_types::file_type::{FileType};
use crate::conversions::simple_copy;
use crate::file_types::document::text::TXT;
use crate::formats::simple_data_format::SimpleDataFormat;

lazy_static! {
    pub static ref HTML: FileType = FileType{
        name: "HTML",
        extensions: set!["htm", "html"],
        conversions: converts_to!{
            TXT => simple_copy
        }
    };
}
