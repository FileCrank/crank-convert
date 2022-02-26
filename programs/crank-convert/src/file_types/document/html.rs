use lazy_static::lazy_static;
use crate::converts_to;
use crate::file_types::file_type::{FileType};
use crate::formats::simple_data::SimpleDataFormat;
use crate::conversions::simple_copy;
use crate::file_types::document::text::TXT;

lazy_static! {
    pub static ref HTML: FileType<SimpleDataFormat> = FileType{
        name: "HTML",
        extensions: vec!["htm", "html"],
        conversions: converts_to!{
            TXT => simple_copy
        }
    };
}
