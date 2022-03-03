use lazy_static::lazy_static;
use crate::converts_to;
use crate::file_types::file_type::FileType;
use crate::formats::simple_data_format::SimpleDataFormat;
use crate::conversions::simple_copy;
use crate::file_types::document::html::HTML;

lazy_static! {
    pub static ref TXT: FileType<SimpleDataFormat> = FileType {
        name: "TXT",
        extensions: vec!["txt", "text"],
        conversions: converts_to!{
            HTML => simple_copy
        }
    };
}