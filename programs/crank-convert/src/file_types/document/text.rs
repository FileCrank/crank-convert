use crate::conversions::simple_copy;
use crate::file_types::document::html::HTML;
use crate::file_types::file_type::FileType;
use crate::formats::data_format::{empty_bytes, initialize_bytes};
use crate::{converts_to, set};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TXT: FileType<'static> = FileType {
        name: "TXT",
        extensions: set!["txt", "text"],
        conversions: converts_to! {
            HTML => simple_copy
        },
        initialize: Box::new(initialize_bytes),
        empty: Box::new(empty_bytes)
    };
}
